#!/usr/bin/env python3
"""A-TownChain SHA-Level Evidence Engine v4.3.10.

Exit 0 = VERIFIED, 1 = FAIL, 2 = BLOCKED.
No SHA fallback, no latest-run fallback, no manual override.
"""
from __future__ import annotations
import argparse, base64, hashlib, json, os, sys, urllib.error, urllib.parse, urllib.request
from dataclasses import dataclass
from datetime import datetime, timezone
from typing import Any, Mapping, Optional, Sequence

class EvidenceBlockedError(RuntimeError): pass
class EvidenceValidationError(RuntimeError): pass

def now():
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00","Z")

def sha1(v, field):
    if not isinstance(v,str) or len(v)!=40 or any(c not in "0123456789abcdefABCDEF" for c in v):
        raise EvidenceValidationError("%s: invalid SHA-1" % field)
    return v.lower()

def sha256(v, field):
    if not isinstance(v,str) or len(v)!=64 or any(c not in "0123456789abcdefABCDEF" for c in v):
        raise EvidenceValidationError("%s: invalid SHA-256" % field)
    return v.lower()

def git_blob_sha(data):
    return hashlib.sha1(("blob %d\0" % len(data)).encode()+data).hexdigest()

class Schema:
    @staticmethod
    def d(v,f):
        if not isinstance(v,dict): raise EvidenceValidationError("%s: expected object" % f)
        return v
    @staticmethod
    def l(v,f):
        if not isinstance(v,list): raise EvidenceValidationError("%s: expected array" % f)
        return v
    @staticmethod
    def s(v,f):
        if not isinstance(v,str) or not v: raise EvidenceValidationError("%s: expected string" % f)
        return v
    @staticmethod
    def i(v,f):
        if isinstance(v,bool) or not isinstance(v,int): raise EvidenceValidationError("%s: expected integer" % f)
        return v

@dataclass(frozen=True)
class Response:
    status:int
    body:bytes
    headers:Mapping[str,str]

class GitHubClient:
    BASE="https://api.github.com"
    def __init__(self,token,timeout=30):
        if not token: raise EvidenceBlockedError("GitHub token unavailable")
        self.token,self.timeout=token,timeout
    def raw(self,path):
        url=path if path.startswith("http") else self.BASE+path
        req=urllib.request.Request(url,headers={
            "Accept":"application/vnd.github+json",
            "Authorization":"Bearer "+self.token,
            "X-GitHub-Api-Version":"2026-03-10",
            "User-Agent":"A-TownChain-Evidence-Engine/4.3.10"})
        try:
            with urllib.request.urlopen(req,timeout=self.timeout) as r:
                return Response(r.status,r.read(),dict(r.headers.items()))
        except urllib.error.HTTPError as e:
            return Response(e.code,e.read(),dict(e.headers.items()) if e.headers else {})
        except (urllib.error.URLError,TimeoutError,OSError) as e:
            raise EvidenceBlockedError("NETWORK_BLOCKED: %s" % e) from e
    def json(self,path):
        r=self.raw(path)
        if r.status<200 or r.status>=300:
            kind={401:"UNAUTHORIZED",403:"FORBIDDEN",404:"NOT_FOUND",429:"RATE_LIMITED"}.get(r.status,"SERVER_ERROR" if r.status>=500 else "HTTP_ERROR")
            raise EvidenceBlockedError("%s: HTTP %s: %s"%(kind,r.status,r.body[:500].decode("utf-8","replace")))
        try:
            return json.loads(r.body.decode("utf-8","strict"))
        except (UnicodeDecodeError,json.JSONDecodeError) as e:
            raise EvidenceValidationError("INVALID_RESPONSE: %s"%e) from e
    def paginated(self,path,key,max_pages=50):
        out=[]; sep="&" if "?" in path else "?"
        for page in range(1,max_pages+1):
            data=Schema.d(self.json("%s%sper_page=100&page=%d"%(path,sep,page)),"page")
            items=Schema.l(data.get(key,[]),key)
            if not items: return out
            out.extend(Schema.d(x,key+" item") for x in items)
            if len(items)<100: return out
        overflow=Schema.d(self.json("%s%sper_page=100&page=%d"%(path,sep,max_pages+1)),"overflow page")
        items=Schema.l(overflow.get(key,[]),"overflow items")
        if items: raise EvidenceBlockedError("PAGINATION_BLOCKED: maximum page limit exceeded for %s"%path)
        return out

@dataclass(frozen=True)
class Target:
    repository:str
    requested_sha:str
    resolved_sha:str
    tree_sha:str
    def __post_init__(self):
        a,b,c=sha1(self.requested_sha,"requested_sha"),sha1(self.resolved_sha,"resolved_sha"),sha1(self.tree_sha,"tree_sha")
        if a!=b: raise EvidenceValidationError("EXACT-SHA violation: requested != resolved")
        object.__setattr__(self,"requested_sha",a); object.__setattr__(self,"resolved_sha",b); object.__setattr__(self,"tree_sha",c)

@dataclass(frozen=True)
class Node:
    id:str
    path:str
    sha:str
    type:str
    metadata:Mapping[str,Any]

@dataclass(frozen=True)
class Edge:
    source:str
    relation:str
    target:str

RELATIONS={
    ("TREE","CONTAINS","BLOB"),("WORKFLOW","BINDS_TO","BLOB"),
    ("RUN","RUNS_WORKFLOW","WORKFLOW"),("RUN","RUNS_COMMIT","COMMIT"),
    ("RUN","HAS_JOB","JOB"),("JOB","HAS_STEP","STEP"),("JOB","HAS_LOG","LOG")
}

class Ledger:
    def __init__(self,target):
        self.target=target; self.nodes={}; self.edges=[]; self.frozen=False
    def add_node(self,n):
        if self.frozen: raise EvidenceValidationError("ledger frozen")
        if n.id in self.nodes: raise EvidenceValidationError("duplicate node: "+n.id)
        self.nodes[n.id]=n
    def add_edge(self,e):
        if self.frozen: raise EvidenceValidationError("ledger frozen")
        s=self.nodes.get(e.source); t=self.nodes.get(e.target)
        if not s or not t: raise EvidenceValidationError("edge references missing node")
        if (s.type,e.relation,t.type) not in RELATIONS: raise EvidenceValidationError("invalid edge: %r"% (e,))
        if e in self.edges: raise EvidenceValidationError("duplicate edge: %r"% (e,))
        self.edges.append(e)
    def freeze(self):
        self.frozen=True

class Inspector:
    def __init__(self,c,o,r): self.c,self.o,self.r=c,o,r
    def p(self,s): return "/repos/%s/%s%s"%(self.o,self.r,s)
    def target(self,requested):
        requested=sha1(requested,"requested_sha")
        d=Schema.d(self.c.json(self.p("/commits/"+requested)),"commit")
        resolved=sha1(d.get("sha"),"commit.sha")
        if resolved!=requested: raise EvidenceValidationError("EXACT-SHA FAIL")
        tree=Schema.d(Schema.d(d.get("commit"),"commit.commit").get("tree"),"commit.tree")
        return Target("%s/%s"%(self.o,self.r),requested,resolved,sha1(tree.get("sha"),"tree.sha"))
    def tree(self,commit):
        d=Schema.d(self.c.json(self.p("/git/trees/%s?recursive=1"%commit)),"tree")
        if d.get("truncated") is not False: raise EvidenceBlockedError("TREE_BLOCKED: truncated or invalid tree")
        entries={}
        for x in Schema.l(d.get("tree"),"tree.tree"):
            x=Schema.d(x,"tree entry"); path=Schema.s(x.get("path"),"tree.path")
            if path in entries: raise EvidenceValidationError("duplicate tree path: "+path)
            typ=Schema.s(x.get("type"),"tree.type")
            if typ not in ("blob","tree"): raise EvidenceValidationError("unsupported tree type: "+typ)
            entries[path]={"type":typ,"sha":sha1(x.get("sha"),path+".sha")}
        returned=sha1(d.get("sha"),"tree.sha")
        return returned,entries
    def blob(self,bsha):
        d=Schema.d(self.c.json(self.p("/git/blobs/"+sha1(bsha,"blob.sha"))),"blob")
        if d.get("encoding")!="base64": raise EvidenceValidationError("blob encoding is not base64")
        try: data=base64.b64decode(Schema.s(d.get("content"),"blob.content"),validate=True)
        except Exception as e: raise EvidenceValidationError("invalid blob base64: %s"%e) from e
        actual=git_blob_sha(data)
        if actual!=bsha: raise EvidenceValidationError("BLOB SHA mismatch: %s != %s"%(bsha,actual))
        return data
    def logs(self,job):
        r=self.c.raw(self.p("/actions/jobs/%d/logs"%job))
        if r.status!=200: raise EvidenceBlockedError("LOG_BLOCKED: HTTP %s for job %d"%(r.status,job))
        return r.body

@dataclass(frozen=True)
class RunSelectionPolicy:
    require_unique_match:bool=True
    explicit_run_ids:Optional[Sequence[int]]=None
    uniqueness_scope:str="PER_WORKFLOW"
    def __post_init__(self):
        if self.uniqueness_scope not in ("PER_WORKFLOW","GLOBAL"):
            raise EvidenceValidationError("invalid uniqueness_scope")

class Adapter:
    PREFIX=".github/workflows/"
    def __init__(self,c,o,r,policy): self.c,self.o,self.r,self.policy=c,o,r,policy
    def p(self,s): return "/repos/%s/%s%s"%(self.o,self.r,s)
    def run(self,ledger,entries,target):
        workflows=self.c.paginated(self.p("/actions/workflows"),"workflows")
        by_path={}; by_id={}
        for w in workflows:
            path=Schema.s(w.get("path"),"workflow.path"); wid=Schema.i(w.get("id"),"workflow.id")
            if wid<=0 or path in by_path or wid in by_id: raise EvidenceValidationError("workflow metadata not unique/valid")
            by_path[path]=w; by_id[wid]=path
        global_runs=set()
        for path,e in entries.items():
            if not(path.startswith(self.PREFIX) and path.endswith((".yml",".yaml")) and e["type"]=="blob"): continue
            bsha=e["sha"]; bid="blob:%s:%s"%(path,bsha)
            if bid not in ledger.nodes: raise EvidenceBlockedError("workflow blob missing: "+path)
            meta=by_path.get(path)
            if not meta: raise EvidenceBlockedError("workflow metadata missing: "+path)
            wid=Schema.i(meta.get("id"),"workflow.id")
            detail=Schema.d(self.c.json(self.p("/actions/workflows/%d"%wid)),"workflow detail")
            if Schema.s(detail.get("path"),"workflow detail.path")!=path or Schema.i(detail.get("id"),"workflow detail.id")!=wid:
                raise EvidenceValidationError("workflow detail binding mismatch: "+path)
            wn="workflow:%d"%wid
            ledger.add_node(Node(wn,path,bsha,"WORKFLOW",{"workflow_id":wid,"workflow_path":path,"workflow_name":meta.get("name"),"state":meta.get("state")}))
            ledger.add_edge(Edge(wn,"BINDS_TO",bid))
            runs=self.c.paginated(self.p("/actions/workflows/%d/runs?head_sha=%s"%(wid,urllib.parse.quote(target.requested_sha,safe=""))),"workflow_runs")
            matches=[]
            for run in runs:
                rid=Schema.i(run.get("id"),"run.id"); head=Schema.s(run.get("head_sha"),"run.head_sha")
                if head!=target.requested_sha: continue
                if self.policy.explicit_run_ids is not None and rid not in self.policy.explicit_run_ids: continue
                rw=run.get("workflow_id",run.get("workflow",{}).get("id") if isinstance(run.get("workflow"),dict) else None)
                if not isinstance(rw,int) or rw!=wid: raise EvidenceValidationError("run workflow binding mismatch: %d"%rid)
                matches.append(run)
            if self.policy.require_unique_match and len(matches)!=1:
                raise EvidenceBlockedError("REQUIRE_UNIQUE_MATCH failed for workflow %d: %d runs"%(wid,len(matches)))
            if not matches: raise EvidenceBlockedError("no exact-SHA run for workflow %d"%wid)
            for run in matches:
                rid=Schema.i(run.get("id"),"run.id")
                if self.policy.uniqueness_scope=="GLOBAL" and rid in global_runs: raise EvidenceValidationError("run selected twice: %d"%rid)
                global_runs.add(rid)
                rn="run:%d"%rid
                ledger.add_node(Node(rn,path,target.resolved_sha,"RUN",{"run_id":rid,"workflow_id":wid,"head_sha":run["head_sha"],"status":run.get("status"),"conclusion":run.get("conclusion"),"event":run.get("event")}))
                ledger.add_edge(Edge(rn,"RUNS_WORKFLOW",wn)); ledger.add_edge(Edge(rn,"RUNS_COMMIT","commit:"+target.resolved_sha))
                jobs=self.c.paginated(self.p("/actions/runs/%d/jobs"%rid),"jobs")
                if not jobs: raise EvidenceBlockedError("run %d contains zero jobs"%rid)
                for job in jobs: self.job(ledger,target,rn,rid,job)
    def job(self,ledger,target,rn,rid,j):
        jid=Schema.i(j.get("id"),"job.id")
        if j.get("run_id") is not None and j.get("run_id")!=rid: raise EvidenceValidationError("job run_id mismatch: %d"%jid)
        jn="job:%d"%jid
        ledger.add_node(Node(jn,"",target.resolved_sha,"JOB",{"job_id":jid,"run_id":rid,"name":Schema.s(j.get("name"),"job.name"),"status":j.get("status"),"conclusion":j.get("conclusion"),"head_sha":j.get("head_sha")}))
        ledger.add_edge(Edge(rn,"HAS_JOB",jn))
        seen=set()
        for idx,s in enumerate(Schema.l(j.get("steps",[]),"job.steps")):
            s=Schema.d(s,"step"); num=Schema.i(s.get("number",idx+1),"step.number")
            if num<=0 or num in seen: raise EvidenceValidationError("duplicate/invalid step: %d"%num)
            seen.add(num); sn="step:%d:%d"%(jid,num)
            ledger.add_node(Node(sn,"",target.resolved_sha,"STEP",{"job_id":jid,"step_number":num,"step_name":Schema.s(s.get("name"),"step.name"),"status":s.get("status"),"conclusion":s.get("conclusion")}))
            ledger.add_edge(Edge(jn,"HAS_STEP",sn))
        data=Inspector(self.c,self.o,self.r).logs(jid); digest=hashlib.sha256(data).hexdigest(); sha256(digest,"log.sha256")
        ln="log:job:%d:%s"%(jid,digest)
        ledger.add_node(Node(ln,"",target.resolved_sha,"LOG",{"job_id":jid,"run_id":rid,"content_bytes":data,"byte_length":len(data),"content_sha256":digest,"retrieval_timestamp":now()}))
        ledger.add_edge(Edge(jn,"HAS_LOG",ln))

class Validator:
    def __init__(self,l,t): self.l,self.t=l,t
    def ci(self):
        if "commit:"+self.t.resolved_sha not in self.l.nodes: raise EvidenceValidationError("missing commit node")
        for n in self.l.nodes.values():
            incoming=lambda rel:[e.source for e in self.l.edges if e.relation==rel and e.target==n.id]
            outgoing=lambda rel:[e.target for e in self.l.edges if e.relation==rel and e.source==n.id]
            if n.type=="WORKFLOW" and len(outgoing("BINDS_TO"))!=1: raise EvidenceValidationError("workflow binding invalid: "+n.id)
            if n.type=="RUN":
                if len(outgoing("RUNS_WORKFLOW"))!=1 or len(outgoing("RUNS_COMMIT"))!=1: raise EvidenceValidationError("run bindings invalid: "+n.id)
                if n.metadata.get("head_sha")!=self.t.resolved_sha: raise EvidenceValidationError("run head_sha mismatch: "+n.id)
            if n.type=="JOB" and len(incoming("HAS_JOB"))!=1: raise EvidenceValidationError("job parent invalid: "+n.id)
            if n.type=="STEP":
                p=incoming("HAS_STEP")
                if len(p)!=1 or p[0]!="job:%s"%n.metadata.get("job_id"): raise EvidenceValidationError("step parent invalid: "+n.id)
            if n.type=="LOG":
                p=incoming("HAS_LOG"); content=n.metadata.get("content_bytes"); digest=n.metadata.get("content_sha256")
                if len(p)!=1 or p[0]!="job:%s"%n.metadata.get("job_id"): raise EvidenceValidationError("log parent invalid: "+n.id)
                if not isinstance(content,bytes) or hashlib.sha256(content).hexdigest()!=digest: raise EvidenceValidationError("log SHA-256 mismatch: "+n.id)
        if not any(n.type=="WORKFLOW" for n in self.l.nodes.values()): raise EvidenceValidationError("no workflow evidence")
        if not any(n.type=="RUN" for n in self.l.nodes.values()): raise EvidenceValidationError("no run evidence")
    def global_(self):
        for e in self.l.edges:
            if e.source not in self.l.nodes or e.target not in self.l.nodes: raise EvidenceValidationError("orphan edge")
        if not self.l.edges: raise EvidenceValidationError("empty evidence graph")

def main():
    ap=argparse.ArgumentParser()
    ap.add_argument("--repository",required=True)
    ap.add_argument("--requested-sha",required=True)
    ap.add_argument("--output")
    ap.add_argument("--token-env",default="GITHUB_TOKEN")
    a=ap.parse_args()
    try:
        requested=sha1(a.requested_sha,"requested_sha")
        if "/" not in a.repository: raise EvidenceValidationError("repository must be OWNER/REPO")
        owner,repo=a.repository.split("/",1); token=os.environ.get(a.token_env)
        if not token: raise EvidenceBlockedError("GitHub token unavailable")
        c=GitHubClient(token); ins=Inspector(c,owner,repo); target=ins.target(requested)
        returned_tree,entries=ins.tree(target.resolved_sha)
        if returned_tree!=target.tree_sha: raise EvidenceValidationError("tree SHA mismatch")
        l=Ledger(target)
        l.add_node(Node("commit:"+target.resolved_sha,"",target.resolved_sha,"COMMIT",{"requested_sha":target.requested_sha,"resolved_sha":target.resolved_sha,"tree_sha":target.tree_sha}))
        l.add_node(Node("tree:"+target.tree_sha,"",target.tree_sha,"TREE",{"commit_sha":target.resolved_sha}))
        for path,e in entries.items():
            if e["type"]!="blob": continue
            data=ins.blob(e["sha"]); bid="blob:%s:%s"%(path,e["sha"])
            l.add_node(Node(bid,path,e["sha"],"BLOB",{"byte_length":len(data)})); l.add_edge(Edge("tree:"+target.tree_sha,"CONTAINS",bid))
        Adapter(c,owner,repo,RunSelectionPolicy()).run(l,entries,target)
        v=Validator(l,target); v.ci(); v.global_(); l.freeze()
        result={"repository":a.repository,"requested_sha":target.requested_sha,"resolved_sha":target.resolved_sha,"tree_sha":target.tree_sha,"status":"VERIFIED","exit_code":0,"evidence_frozen":l.frozen,"timestamp":now()}
    except EvidenceBlockedError as e:
        result={"repository":a.repository,"requested_sha":a.requested_sha,"status":"BLOCKED","exit_code":2,"evidence_frozen":False,"error":str(e),"timestamp":now()}
    except EvidenceValidationError as e:
        result={"repository":a.repository,"requested_sha":a.requested_sha,"status":"FAIL","exit_code":1,"evidence_frozen":False,"error":str(e),"timestamp":now()}
    except Exception as e:
        result={"repository":a.repository,"requested_sha":a.requested_sha,"status":"FAIL","exit_code":1,"evidence_frozen":False,"error":"INTERNAL: %s: %s"%(type(e).__name__,e),"timestamp":now()}
    text=json.dumps(result,indent=2,sort_keys=True,ensure_ascii=False)
    print(text)
    if a.output:
        with open(a.output,"w",encoding="utf-8",newline="\n") as f: f.write(text+"\n")
    return result["exit_code"]
if __name__=="__main__": raise SystemExit(main())

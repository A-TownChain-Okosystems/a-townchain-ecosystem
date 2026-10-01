#!/usr/bin/env python3
"""Deterministic-friendly runtime evidence collector.

Measures wall-clock duration of explicitly supplied build/test/conformance commands.
It never changes gates or turns missing commands into PASS.
"""

from __future__ import annotations
import argparse, json, subprocess, time
from datetime import datetime, timezone


def run_step(name: str, command: str) -> dict:
    start = time.monotonic_ns()
    proc = subprocess.run(command, shell=True, text=True)
    duration_ms = (time.monotonic_ns() - start) // 1_000_000
    return {
        "step": name,
        "command": command,
        "exit_code": proc.returncode,
        "duration_ms": duration_ms,
    }


def main() -> int:
    p = argparse.ArgumentParser()
    p.add_argument("--source-sha", required=True)
    p.add_argument("--build", required=True)
    p.add_argument("--test", required=True)
    p.add_argument("--protocol-conformance", required=True)
    p.add_argument("--runtime-gas", required=True)
    p.add_argument("--library-conformance", required=True)
    p.add_argument("--run-id")
    p.add_argument("--job-id")
    p.add_argument("--workflow")
    args = p.parse_args()

    commands = [
        ("build", args.build),
        ("test", args.test),
        ("protocol_conformance", args.protocol_conformance),
        ("runtime_gas", args.runtime_gas),
        ("library_conformance", args.library_conformance),
    ]
    evidence = [run_step(*item) for item in commands]
    durations = {e["step"]: e["duration_ms"] for e in evidence}
    results = {
        e["step"]: ("PASS" if e["exit_code"] == 0 else "FAIL")
        for e in evidence
    }
    record = {
        "schema_version": "1.0.0",
        "source_sha": args.source_sha,
        "recorded_at": datetime.now(timezone.utc).isoformat(),
        "run_id": args.run_id,
        "job_id": args.job_id,
        "workflow": args.workflow,
        "measurements": {
            "build_ms": durations["build"],
            "test_ms": durations["test"],
            "protocol_conformance_ms": durations["protocol_conformance"],
            "runtime_gas_ms": durations["runtime_gas"],
            "library_conformance_ms": durations["library_conformance"],
        },
        "results": results,
        "evidence": evidence,
    }
    print(json.dumps(record, sort_keys=True, separators=(",", ":")))
    return 0 if all(e["exit_code"] == 0 for e in evidence) else 1


if __name__ == "__main__":
    raise SystemExit(main())

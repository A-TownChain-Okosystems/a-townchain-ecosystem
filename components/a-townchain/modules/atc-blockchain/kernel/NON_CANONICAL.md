# This crate is not canonical and does not build

**Stand:** 2026-11-14 · **Entscheidung:** nicht reparieren, E4/E5

`src/blockchain.rs` ist ein Auszug: eine minifizierte 57-Zeilen-Datei, die
`Block`, `BlockChain` und `ProposalPipeline` definiert und dabei
`crate::mempool`, `crate::consensus` sowie `crate::security` importiert —
Module, die es in diesem Crate nie gab. Dazu nutzt sie `alloc` und `spin`,
ohne die Abhängigkeiten zu deklarieren. Das Crate ist deshalb nicht
übersetzbar und seit 2026-11-14 aus dem Root-Workspace ausgeschlossen.

## Warum hier nichts repariert wird

Dieselben drei Typen stehen kanonisch in **globus-os**:
`modules/atc-shivacore/service_space/src/blockchain.rs` (505 Zeilen,
dokumentiert, übersetzt, im Testlauf grün) mit `mempool.rs` und
`service_space/src/lib.rs` als Nachbarschaft. Nach **E4** (GlobusOS ist die
Kernel-SSOT) und **E5** (Kernel-Autorität) ist die Kopie hier eine
Duplikation. Sie mit Modulen aus dem Archivsnapshot
`components/a-townchain-os-docs/docs/archive/monorepo-full/…/atc-shivacore/kernel`
zu vervollständigen, würde die Duplikation festschreiben, nicht beheben.

## Offene Entscheidung (Owner)

Löschen (Crate entfernen, `atc-blockchain` bleibt als Python/TS-Modul
bestehen) oder als reinen Dokumentationsstand markieren und aus jedem Build
heraushalten. Bis dahin bleibt der Ausschluss im Root-`Cargo.toml` bestehen.

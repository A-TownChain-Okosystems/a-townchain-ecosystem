// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
// ATC-STD-600/Welle 3 (SCR-0128 G4): normative Voraus-Implementierungen (Gate,
// Gas-Tabelle, Chain-ID, Ressourcen-Limits) sind bewusst gehalten und werden mit
// der ATC-VM-ABI-Anbindung aktiviert; kein totes Zufalls-Code-Dead.
#![allow(dead_code)]
//! ATC-VM — Stack-Maschine MVP plus ATC-STD-600 execution gate.

pub mod context;
pub mod ops;
pub mod vm;

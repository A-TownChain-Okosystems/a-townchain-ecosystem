# Ecosystem SDK Architecture
The ecosystem exposes seven SDK domains: ATC-Lang, ATC-Standards, A-TownChain, Aurora-AI, ShivaCore, GlobusOS and Genesis Engine.
Canonical ownership remains with each domain repository. This repository integrates them; it must not fork canonical protocol implementations.
Dependency direction: Standards -> Language -> Chain/VM -> Kernel -> OS/AI -> Engine.

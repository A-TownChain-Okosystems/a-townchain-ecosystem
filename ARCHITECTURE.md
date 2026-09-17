# A-TownChain Ecosystem — System-Architektur

## Schichtenmodell

```
+--------------------------------------------------+
| Anwendungen (Genesis Chronicles, Flagship)      |
+--------------------------------------------------+
| Genesis Engine  (components/genesis-engine)     |
+--------------------------------------------------+
| Aurora AI       (components/aurora-ai)           |
+--------------------------------------------------+
| A-TownChain L1 (components/a-townchain)          |
|   ├─ ATC-VM          components/vm               |
|   ├─ ATC-Algorithm   components/algorithm        |
|   ├─ Node/Wallet/SDK ... 13 Komponenten          |
+--------------------------------------------------+
| ATCLang         (components/atclang)             |
+--------------------------------------------------+
| ShivaCore-Kernel (components/atc-shivacore, TCB) |
+--------------------------------------------------+
```

## Integrationsprinzipien

- **SSOT-Disziplin**: Jedes Quell-Repo bleibt kanonisch für seine Domäne; der
  Umbrella ist Integrationsschicht, kein Ersatz (MIGRATION_MANIFEST.yaml).
- **Sync**: `git subtree pull --prefix=components/<name> <repo> main`.
- **Evidence-First**: Komponenten-Gates laufen komponenten-scoped mit deren
  eigenen Allowlists; das Dach auditiert gegen die Org-Registry.
- **Bekannter Zustand**: genesis-engine main ist bis Merge von PR #13 rot
  (partielle Franchise-Factory-Integration, 17.09.); danach subtree pull.

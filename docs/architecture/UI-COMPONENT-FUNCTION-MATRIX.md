# A-TownChain Ecosystem — UI Component & Function Matrix

**Document ID:** ATC-UI-MATRIX-001  
**Status:** ARCHITECTURE CONTRACT  
**Scope:** canonical UI layer across Frontend, OS, Aurora, A-TownChain, Wallet, Marketplace, NFT, Genesis and GateToHell.  
**Rule:** UI presence never proves implementation, authorization, backend integration or verification.

## 1. Canonical Traceability
Every UI capability MUST be traceable as:
```
Vision → UI Domain → Component → Function / Interaction → UI Contract
→ Source → Component Test → Integration / E2E Test → Workflow
→ Exact-SHA Evidence → Verification → Residual
```
Evidence separation: `Error Evidence ≠ Finding Evidence ≠ Verification Evidence`.
Status: `UNANALYZED → ANALYZED → FIXED → RERUNNING → VERIFIED / RESIDUAL`

## 2. Canonical UI Domains

| ID | Domain | Canonical component set | Canonical functions / interactions |
|---|---|---|---|
| UI-01 | App Shell | AppShell, AppFrame, Header, Footer | bootstrap, lifecycle, global providers, error boundary |
| UI-02 | Navigation | Nav, Sidebar, Tabs, Breadcrumbs | navigate, back, forward, deep-link, active-state |
| UI-03 | Layout | Container, Grid, Stack, SplitPane | responsive layout, sizing, alignment, collapse |
| UI-04 | Typography | Heading, Text, Label, Code, Link | semantic text, truncation, copy, link |
| UI-05 | Buttons | Button, IconButton, ButtonGroup | invoke action, disabled/loading, keyboard activation |
| UI-06 | Inputs | TextField, NumberField, PasswordField, SearchField | input, validation, masking, clear, submit |
| UI-07 | Selection | Checkbox, Radio, Switch, Select, MultiSelect | select, toggle, multi-select, validation |
| UI-08 | Forms | Form, Field, FieldGroup, FormActions | validate, dirty-state, submit, reset, error mapping |
| UI-09 | Data Display | Card, List, Table, DataGrid, Tree | render, sort, filter, select, expand, virtualize |
| UI-10 | Feedback | Alert, Banner, Toast, Snackbar | success/error/info/warning and dismissal |
| UI-11 | Dialogs | Dialog, Modal, ConfirmDialog | open, confirm, cancel, escape, focus trap |
| UI-12 | Overlays | Popover, Tooltip, Dropdown, ContextMenu | open, anchor, keyboard navigation, dismiss |
| UI-13 | Progress | Spinner, ProgressBar, Skeleton | loading, determinate progress, completion |
| UI-14 | Empty/Error | EmptyState, ErrorState, Retry | empty, retry, recovery, fallback |
| UI-15 | Search | SearchBox, FilterBar, ResultList | query, suggest, filter, clear, select |
| UI-16 | Pagination | Pagination, InfiniteScroll, LoadMore | page, cursor, append, restore |
| UI-17 | Notifications | NotificationCenter, NotificationItem, Badge | receive, read, dismiss, navigate |
| UI-18 | Authentication | Login, Registration, Recovery, MFA, Passkey | register, authenticate, logout, recover, verify |
| UI-19 | Identity | Profile, Avatar, AccountSwitcher, SessionList | identity selection, account switch, sessions |
| UI-20 | Security | PermissionPrompt, CapabilityPrompt, SecurityWarning | request, explain, approve, deny, revoke |
| UI-21 | Wallet | WalletConnect, AccountSelector, SigningPrompt | connect, select account, sign, reject |
| UI-22 | Transactions | TxBuilder, TxPreview, TxConfirm, TxStatus | build, validate, preview, sign, submit, track |
| UI-23 | Blockchain | BlockView, TxView, ValidatorView, StakingView | inspect blocks, transactions, validators, stake |
| UI-24 | Tokens | TokenCard, TokenList, TransferForm, AssetDetails | discover, inspect, transfer, balance |
| UI-25 | NFTs | NFTCard, Gallery, CollectionView, MetadataView | inspect, filter, transfer, metadata, ownership |
| UI-26 | Marketplace | Listing, Offer, Cart, Checkout, OrderStatus | list, search, offer, purchase, settle, inspect |
| UI-27 | Aurora AI | Chat, AgentPanel, ModelSelector, ToolApproval | chat, model selection, agent, tool approval |
| UI-28 | Genesis | WorldSelector, Character, Inventory, Quest, HUD | world, character, quest, inventory |
| UI-29 | GateToHell Browser | AddressBar, Tabs, WebView, Downloads, Permissions | navigate, origin isolation, tabs, downloads |
| UI-30 | OS | Settings, DeviceManager, NetworkPanel, StoragePanel | inspect/configure authorized system services |
| UI-31 | Media | ImageViewer, VideoPlayer, AudioPlayer, Gallery | play, pause, seek, zoom, fullscreen |
| UI-32 | Editor | Inspector, PropertyPanel, AssetBrowser, Timeline | inspect/edit assets and sequences |
| UI-33 | Developer Tools | Console, Inspector, NetworkPanel, DebugPanel | inspect, trace, replay, diagnose |
| UI-34 | Accessibility | FocusManager, A11yTree, KeyboardNav | focus, keyboard traversal, semantic exposure |
| UI-35 | Responsive | MobileLayout, TabletLayout, DesktopLayout | breakpoint and input adaptation |
| UI-36 | Theme | ThemeProvider, TokenProvider, ThemeSelector | theme switch, token resolution, persistence |
| UI-37 | Internationalization | LocaleProvider, LanguageSelector | locale switch, translation, formatting |
| UI-38 | Animation | Transition, Motion, Microinteraction | enter/exit, state transition, reduced-motion |
| UI-39 | Drag & Drop | Draggable, DropZone, SortableList | drag, drop, reorder, cancel |
| UI-40 | Files | FilePicker, Upload, Download, Preview | select, validate, upload, download, preview |
| UI-41 | Real-Time | LiveIndicator, Stream, Presence, EventFeed | subscribe, stream, reconnect, update |
| UI-42 | Visualization | Chart, Graph, Timeline, Metrics | render, zoom, filter, inspect |
| UI-43 | Spatial | Map, Marker, WorldMap, SpatialOverlay | pan, zoom, select, spatial interaction |
| UI-44 | Commands | CommandPalette, ShortcutRegistry | search and execute commands, shortcuts |
| UI-45 | Guidance | Onboarding, Tutorial, HelpPanel, ContextHelp | guide, step, dismiss, resume |
| UI-46 | Privacy | ConsentDialog, PrivacySettings, DataControls | consent, retention, export/delete |
| UI-47 | Audit | ActivityLog, AuditEntry, EvidenceViewer | inspect event history and evidence metadata |
| UI-48 | UI State | StateBoundary, DirtyState, ValidationState | loading, valid, invalid, dirty, disabled |
| UI-49 | Component Infrastructure | ComponentRegistry, Portal, UIProvider | registration, dependency injection, portal rendering |
| UI-50 | UI Testing | ComponentHarness, TestFixture, VisualFixture | component, interaction, visual and E2E tests |
| UI-51 | Build / Release | UI Build, Asset Pipeline, Artifact | compile, bundle, hash, package, publish |
| UI-52 | Verification | EvidencePanel, VerificationStatus | bind exact-SHA evidence, show verification/residual |
| UI-53 | Governance | ComponentOwner, DesignRule, Deprecation | ownership, versioning, deprecation, compatibility |
| UI-54 | UI Security | CSP boundary, TrustedRenderer, SandboxBoundary | trusted rendering, origin boundary, sanitization |
| UI-55 | Observability | ErrorReporter, PerformanceMonitor, TracePanel | errors, latency, interaction traces, diagnostics |

## 3. Canonical UI Component Hierarchy
```
Design Tokens → Primitive Components → Composite Components
→ Interaction Patterns → Views / Pages → Application Shell
→ Frontend State / Data → Authorized System Boundary
```

## 4. Security-Critical UI Invariants
1. UI state never grants authority.
2. AI output never grants authority.
3. Browser content never grants authority.
4. Wallet signing requires explicit authorization.
5. Blockchain state is authoritative; UI/indexer state is derived.
6. OS operations require capability/policy authorization.
7. Marketplace purchase does not itself prove settlement.
8. NFT presentation metadata does not determine ownership.
9. Loading/success indicators are not verification evidence.
10. Client-side validation never replaces authoritative validation.
11. Secrets/private keys must never enter UI telemetry, logs or evidence.
12. Error displays are not findings or verification evidence.
13. Accessibility implementation must be tested.
14. Security-sensitive UI fails closed when authorization state is unavailable.

## 5. Standard UI Contract
Each component/function record SHOULD contain:
```yaml
id:
domain:
component:
function:
interaction:
inputs:
outputs:
state:
errors:
authorization:
security_boundary:
contract:
source:
test:
workflow:
exact_sha:
verification:
residual:
status:
```
No stage may be inferred from a previous stage.

## 6. Cross-System UI Boundaries
| Boundary | UI role | Authoritative layer |
|---|---|---|
| UI → Identity | collect/display identity | Identity service |
| UI → Aurora | request AI/agent action | Capability + Policy + Approval |
| UI → Wallet | request signing | Wallet/key boundary |
| UI → Blockchain | submit/read protocol operations | Node/protocol state |
| UI → Marketplace | request commerce action | Marketplace contract/service |
| UI → NFT | request lifecycle action | NFT contract/state |
| UI → Genesis | request gameplay action | Genesis authoritative runtime |
| UI → GateToHell | request browser action | Browser security boundary |
| UI → GlobusOS | request system operation | OS capability/policy boundary |
| UI → ShivaCore | indirect only | OS/kernel security boundary |

## 7. UI Verification Pipeline
```
Source → Component Build → Type/Static Checks → Unit/Component Tests
→ Accessibility Tests → Integration Tests → E2E Tests → Security Tests
→ Exact-SHA CI → Artifact/Log Evidence → Verification → Residual
```

## 8. Same Detail Rule for All Other Matrices
Every existing system matrix is expanded using:
```
Domain → Component → Function/API → Inputs/Outputs → State
→ Contract → Security Boundary → Source → Test → Workflow
→ Exact-SHA Evidence → Verification → Residual
```
The repository-wide AST/Symbol Inventory is the discovery mechanism. It does not prove correctness or verification.

## 9. Residuals
1. UI architecture does not prove component existence.
2. AST inventory does not prove semantic correctness.
3. Frontend/backend integration requires repository evidence.
4. Accessibility/security/E2E claims require actual test evidence.
5. Exact-SHA verification is commit-specific.
6. UI domains must be mapped to concrete repositories before implementation status is assigned.

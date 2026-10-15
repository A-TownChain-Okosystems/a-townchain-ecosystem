# GlobusOS Guest Operating System Virtualization — Architecture Specification 001

- **Specification ID:** `GLOBUSOS-GUEST-OS-VIRTUALIZATION-001`
- **Version:** `0.1.0-draft`
- **Status:** `PROPOSED — NOT IMPLEMENTED OR VERIFIED`
- **Date:** `2026-10-15`
- **Canonical implementation owner:** `A-TownChain-Okosystems/globus-os`
- **Architecture/evidence registry:** `A-TownChain-Okosystems/a-townchain-ecosystem`

## 1. Goal

Allow users to install and run a licensed Windows or Linux guest operating system inside GlobusOS as a separately managed second system, with a virtual disk, guest boot firmware, virtual devices, and an independent guest lifecycle. The user should be able to switch between the GlobusOS desktop and the guest OS without rebooting the physical host, similar to running Windows in a virtual machine on macOS.

This is a **hosted virtual machine**, not the same as native dual boot. The guest OS owns its guest kernel and userspace inside the virtual machine; it does not replace ShivaCore or gain direct control over the physical host.

## 2. Deployment modes must remain distinct

### Mode A — Hosted VM (primary target)

GlobusOS remains running. A guest manager creates an isolated virtual machine, attaches a virtual disk and installation ISO, boots virtual firmware, and displays the guest through a window or full-screen console. After installation, the VM boots from its virtual disk. Users can pause/resume, shut down, snapshot, back up, and remove the guest subject to policy.

### Mode B — Native dual boot (optional future capability)

Windows/Linux is installed to a dedicated physical disk or partition and selected through a boot manager before GlobusOS starts. Only use this mode when firmware, hardware, boot-chain, storage layout, driver availability, and platform security requirements are explicitly supported. It is **not emulation**, does not run simultaneously with GlobusOS, and is not part of the initial VM milestone.

### Mode C — Full system emulation (fallback / cross-architecture)

If guest and host CPU architectures differ or hardware virtualization is unavailable, a system emulator may emulate the CPU and devices. Expect substantially lower performance. Do not describe CPU emulation, hardware-assisted virtualization, and native dual boot as interchangeable.

## 3. Proposed architecture

```text
GlobusOS Desktop / Window Manager
             |
       Guest OS Manager
             |
   Capability + Policy Broker
             |
  Virtual Machine Monitor Boundary
     |       |        |        |
   vCPU    vMemory  vStorage  vNetwork
     |       |        |        |
 Virtual firmware / virtual devices
             |
    Windows or Linux Guest OS
```

- **Guest OS Manager:** UI and lifecycle control for create/import/install/start/pause/resume/shutdown/snapshot/export/delete.
- **Virtualization backend:** narrow adapter to a supported hypervisor or system emulator; select backend per hardware/architecture. QEMU is a potential evaluation candidate, not a selected dependency.
- **Virtual machine monitor boundary:** isolate each guest process and its memory, device model, and virtual resources from unrelated host services.
- **Virtual firmware and devices:** versioned virtual machine profile, boot firmware, storage, display, keyboard/pointer, network, clock, TPM/vTPM where supported, and guest-agent channel.
- **Policy broker:** explicit capability grants for disk images, shared folders, clipboard, USB/device passthrough, networking, audio, GPU acceleration, and host integration.
- **Desktop integration:** guest console as a movable/resizable/minimizable/maximizable window; full-screen mode; host/guest input capture and release; clear indication of guest identity and state.
- **Storage:** per-guest virtual disk and metadata, integrity checks, quotas, snapshots and backup/restore; guest disk is not mounted read-write by the host while the guest is running unless a safe, explicit protocol allows it.

## 4. Resource and security contract

1. Guest CPU, RAM, virtual disk, vCPU count, network bandwidth and device access are explicitly configured and quota-limited.
2. Guest execution never receives ambient host capabilities. Access to host files, clipboard, USB, camera, microphone, GPU, network bridges, and shared folders is disabled by default and granted individually.
3. Guest memory and device-model boundaries are isolated. A guest crash or forced termination must not corrupt unrelated host services or other guest state.
4. Virtualization backend runs with the least privilege supported by the host. Device models and management interfaces are treated as attack surface.
5. VM management endpoints are local-only by default, authenticated to the logged-in user, and inaccessible to guests unless explicitly exposed.
6. VM images are untrusted input. Validate format/size, enforce quotas, and protect against path traversal, malformed images, resource exhaustion, and snapshot/restore inconsistency.
7. Secure Boot/vTPM claims are made only for tested virtual firmware, key provisioning and guest combinations. Guest Secure Boot is distinct from GlobusOS host Secure Boot.
8. Network mode is explicit: isolated, NAT, or bridged only when supported and authorized. No implicit bridge into privileged GlobusOS/ATC control networks.
9. Wallet keys, ShivaCore authority, host secrets, and ATC validator/node signing keys are never automatically exposed to guests. Any deliberate integration uses a narrow authenticated IPC/API with explicit user approval and audit.
10. A guest cannot alter host boot configuration, physical partitions, Secure Boot policy, or other guest images through the default VM interface.

## 5. Installation and lifecycle

1. User selects **Create virtual machine** and chooses Windows or Linux, guest architecture, resource limits, and storage location.
2. User supplies an installation ISO or obtains it from a trusted, authorized source; license and distribution terms remain the user's responsibility.
3. The manager validates the image, creates a virtual disk, selects a supported machine profile and virtual firmware, then boots the installer.
4. The guest installer operates only on the virtual disk by default; physical disks are not attached implicitly.
5. After installation, the guest boots from its virtual disk and can install compatible guest drivers/tools.
6. The user may pause, resume, shut down, snapshot, export, restore, or delete the VM. Snapshot consistency and rollback semantics must be explicit.
7. Uninstalling the guest removes only the selected VM artifacts after confirmation; it must not modify the GlobusOS boot chain or other disks.

## 6. Compatibility and support matrix

The implementation must publish an explicit tested matrix covering:

- Host CPU architecture and virtualization extensions.
- Backend and version, virtual machine profile, firmware and device model versions.
- Guest OS edition/version, architecture, and installation mode.
- GPU/display acceleration, audio, USB passthrough, networking, suspend/resume, snapshots and guest tools.
- Licensing restrictions, unsupported features, performance limitations and known security residuals.

Do not promise Windows-on-ARM, x86 Windows on ARM hosts, GPU passthrough, vTPM, Secure Boot, suspend/resume, or arbitrary device passthrough without target-specific evidence. When host and guest CPU architectures differ, state whether the path uses CPU emulation or is unsupported.

## 7. MVP boundary

**MVP candidate:** one supported host architecture; one backend selected after evaluation; isolated VM process; virtual disk; installation ISO boot; virtual display/input; NAT or isolated networking; explicit CPU/RAM/disk limits; start/stop lifecycle; guest window integration; safe delete; exact-head integration tests.

**Not MVP by default:** native dual boot management, physical disk passthrough, bridged networking, GPU passthrough, USB passthrough, nested virtualization, live migration, automatic guest license acquisition, and cross-architecture high-performance translation.

## 8. Required verification gates

Before marking any function `VERIFIED`, bind tests to an exact 40-character source SHA and provide workflow run → job → step → terminal conclusion → exit code → logs/artifacts.

Required test families:

- VM creation, installation ISO boot, installation, reboot, shutdown and restart.
- Isolation between guest and host; denial of ungranted file/device/network access.
- Malformed ISO/disk/config inputs, quota exhaustion, memory/CPU pressure, and guest crash recovery.
- Snapshot/restore consistency, backup/restore, safe deletion and path validation.
- Window manager focus, resize, minimize/maximize, full-screen, keyboard/mouse capture and release.
- Network mode boundaries and attempts to reach protected host/control-plane services.
- No guest access to host wallet keys, kernel capabilities, validator secrets, or privileged ATC interfaces.
- Cross-platform results for each explicitly supported host/hardware combination.
- Independent security review of the virtualization backend integration, device model, management API and capability broker.

Green general CI, source-file presence, or a VM boot on one developer machine does not establish production readiness.

## 9. Architecture decisions still required

- Which backend is selected after evaluating licensing, maintainability, device models, performance, isolation and supported architectures?
- Which host CPU/firmware/hardware combinations are first-class targets?
- Which Windows and Linux guest versions are in scope?
- What isolation mechanism can ShivaCore/GlobusOS enforce for the backend process?
- Which virtual firmware and device models are supported?
- What are the resource defaults and hard limits?
- Which guest integrations are permitted by default?
- Is native dual boot a separately approved future project?
- What signing, update, rollback, recovery and security-response process applies to the virtualization backend?

Until resolved and evidenced, this remains an architecture proposal. It does not claim that GlobusOS currently runs Windows/Linux guests or that a virtualization backend is implemented.

## 10. Reference material

- [QEMU system emulation documentation](https://www.qemu.org/docs/master/system/index.html) — describes full-system emulation and supported virtualization accelerators.
- [QEMU system emulation introduction](https://www.qemu.org/docs/master/system/introduction.html) — explains that hardware acceleration depends on host OS and architecture.
- [Apple Support: Windows on Mac](https://support.apple.com/de-de/guide/mac-help/mh11850/mac) — documents the distinct Boot Camp dual-boot model on Intel-based Macs.

These references are architectural background only; they do not select QEMU, prove GlobusOS compatibility, or imply implementation readiness.

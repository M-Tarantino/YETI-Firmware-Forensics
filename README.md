# YETI — Firmware Forensics

> **Project Status: ON HOLD (v0.2 Alpha)**  
> The code is functional, but active development is paused.

---

## What is YETI?

YETI is a high-performance firmware forensics orchestrator built in Rust — designed to solve the "monolithic bottleneck" of traditional tools. Instead of blindly extracting everything, YETI operates with **Surgical Extraction**.

### Core Concepts

- **Virtual Filesystem (VFS):** Browse firmware structures before a single byte is written to disk.
- **On-the-Fly Transformation:** Fix proprietary vendor headers (e.g., modified SquashFS magic bytes) in-memory.
- **Orchestration:** Seamlessly integrate binwalk-rs and backhand into a unified, remote-capable workflow.

---

## Architecture

| Layer | Responsibility |
|-------|---------------|
| **Intelligence Layer** | Vendor identification via DNA-matching, decides which fixups are needed |
| **VFS Layer** | High-speed mapping using `memmap2` for instant access to multi-GB images |
| **Distributed Node** | Server-side heavy decompression, client remains lightweight |

### Tech Stack

- **Language:** Rust (Zero-cost abstractions, Memory safety)
- **Engines:** Binwalk-rs, Backhand (Customized)
- **UI:** Ratatui (Terminal UI)
- **Strategy:** Range-based extraction to minimize I/O overhead

---

## Current State (v0.2 Alpha)

The foundation is in place. The core architecture is functional, but there's still a long way to a stable release.

### What Works
- ✅ Physical offset mapping for granular data access
- ✅ Binwalk-rs core as primary discovery engine
- ✅ TUI-based surgical extraction (`x` command) for individual files

### What's Missing / Planned
- ⏸️ Headless JSON-RPC API for external AI-agent integration
- ⏸️ Automated "Vendor-Fixer" modules for major brands
- ⏸️ Stability, error handling, documentation

---

## Why On Hold?

I'm currently working on **Projekt Helix** — an x86-to-ARM binary translator. My full attention is focused there.

YETI remains my intellectual property. The code is released as-is. There is no active roadmap, no guaranteed bugfixes, and no support commitment. Contributions are welcome but may not be reviewed or merged promptly.

---

## License

### Non-Commercial
- Free use with **Attribution** (Namensnennung)
- Modifications allowed, but must be published under the same license terms
- For research, education, hobby projects, non-profit security research

### Commercial
- **Not permitted** without a separate agreement
- For commercial interest: contact me for an individual license

The code remains my intellectual property. This license does not transfer copyright.

See [LICENSE](LICENSE) for the full legal text.

---

## Contact

Interested in a commercial license or a deep dive into the architecture?

- **Technical topics:** VFS Inode-Mapping, Client/Server Binary Protocol, Vendor-DNA Detection
- **License inquiries:** muneeb.tarantino.job@gmail.com

> *"Don't just scan it. Orchestrate it."*

---

**Author:** Muneeb Tarantino (M-Tarantino)  
**Location:** Deutschland

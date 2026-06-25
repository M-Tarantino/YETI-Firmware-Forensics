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

## Usage

### Prerequisites

- Rust toolchain (stable, >= 1.75)
- SQLite development libraries (for `rusqlite`)

### Build

```bash
# Clone the repository
git clone https://github.com/M-Tarantino/YETI-Firmware-Forensics.git
cd YETI-Firmware-Forensics

# Build the project
cargo build --release

# Generate the forensic DNA database
python3 yeti_dna_generator.py
```

### Run

```bash
# Launch the TUI for interactive firmware analysis
cargo run --release -- /path/to/firmware.bin

# Start the forensic explorer shell (after TUI scan)
# Inside the TUI, press 'e' to enter the explorer

# Start a distributed compute node
cargo run --release -- --node --port 8080

# Send a forensic task to a remote node
cargo run --release -- --client --target 127.0.0.1:8080 --file /path/to/firmware.bin
```

### Explorer Shell Commands

Once inside the forensic explorer:

| Command | Description |
|---------|-------------|
| `ls` | List files and directories in the current path |
| `cd <dir>` | Change into a directory |
| `cd ..` | Go back to root |
| `x <file>` | Extract a single file (surgical extraction) |
| `exit` | Return to the main TUI |

### Example Session

```bash
$ cargo run --release -- tplink_archer_c7.bin

[+] YETI Forensic DNA - Analysing: tplink_archer_c7.bin
[+] System Status: Forensic Engine Active
[+] Worker Threads: 8 (Rayon Parallel)

# Press 'e' to enter the explorer

yeti / > ls
  DIR           0  squashfs-root
  FIL        2048  u-boot.bin
  FIL        4096  kernel.lzma

yeti / > cd squashfs-root
yeti /squashfs-root > ls
  FIL        1024  etc/passwd
  FIL        2048  bin/busybox
  DIR           0  usr/lib

yeti /squashfs-root > x etc/passwd
[+] Extracted etc/passwd (1024 bytes) -> ./extracted/etc/passwd

yeti /squashfs-root > exit
```

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
**GitHub:** [https://github.com/M-Tarantino/YETI-Firmware-Forensics](https://github.com/M-Tarantino/YETI-Firmware-Forensics)  
**Location:** Deutschland

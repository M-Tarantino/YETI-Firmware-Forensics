mod core;
mod storage;
mod interface;
mod network;
mod util;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use colored::Colorize;
use crate::util::error::{YetiResult, YetiError};

/// YETI: Enterprise-Grade Firmware Forensics & Distributed Analysis
#[derive(Parser)]
#[command(author, version = "0.2.0", about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging for deep debugging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan a local firmware image for signatures and hidden filesystems
    Scan { 
        file: PathBuf 
    },
    
    /// Enter the interactive Forensic Explorer shell for a detected filesystem
    Explore { 
        file: PathBuf, 
        #[arg(short, long)] 
        offset: u64 
    },
    
    /// Launch the TUI Dashboard for real-time entropy and data visualization
    Tui { 
        file: PathBuf 
    },
    
    /// Start this instance as a Forensic Compute Node (Worker)
    Server { 
        #[arg(short, long, default_value_t = 8080)] 
        port: u16 
    },

    /// Dispatch a firmware image to a remote YETI node for off-site analysis
    RemoteScan { 
        #[arg(help = "IP:PORT of the remote YETI node")]
        target: String, 
        file: PathBuf 
    },
}

#[tokio::main]
async fn main() -> YetiResult<()> {
    let cli = Cli::parse();

    // Initialize the logger with the requested verbosity
    util::logger::init(if cli.verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    });

    log::info!("{}", "YETI Forensic DNA Suite Initializing...".bold().cyan());

    // Connect to the Knowledge Base (SQLite)
    let kb = storage::KnowledgeBase::new("yeti_dna.db")?;

    match cli.command {
        Commands::Scan { file } => {
            log::info!("Starting local forensic scan: {:?}", file);
            let sigs = kb.load_signatures(None)?;
            let scanner = core::scanner::Scanner::new(sigs);
            
            // Memory map the file for high-performance zero-copy scanning
            let mmap = unsafe { 
                memmap2::Mmap::map(&std::fs::File::open(&file)?)
                    .map_err(|e| YetiError::Io(e))? 
            };
            
            let results = scanner.scan_parallel(&mmap)?;
            interface::reporter::print_summary_table(&results);
        }

        Commands::Explore { file, offset } => {
            log::info!("Mounting Virtual Filesystem at offset 0x{:x}", offset);
            let mmap = unsafe { 
                memmap2::Mmap::map(&std::fs::File::open(&file)?)
                    .map_err(|e| YetiError::Io(e))? 
            };
            
            // Resolve the filesystem and drop into the interactive shell
            let vfs = storage::VirtualFilesystem::new(&mmap, &core::scanner::Candidate { 
                offset, 
                name: "Manual_Mount".into(), 
                score: 1.0 
            })?;
            interface::explorer::start_shell(&vfs)?;
        }

        Commands::Tui { file } => {
            log::info!("Launching TUI Visualizer...");
            interface::tui::launch_ui(file)?;
        }

        Commands::Server { port } => {
            // This runs the asynchronous TCP listener
            network::server::start_compute_node(port).await?;
        }

        Commands::RemoteScan { target, file } => {
            log::info!("Preparing remote dispatch for {:?}", file);
            let data = std::fs::read(file)?;
            network::client::send_forensic_task(&target, &data).await?;
            log::info!("{}", "Remote task completed successfully.".green());
        }
    }

    Ok(())
}
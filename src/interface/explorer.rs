use crate::storage::VirtualFilesystem;
use crate::util::error::YetiResult;
use std::io::{self, Write};
use colored::Colorize;

pub fn start_shell(vfs: &VirtualFilesystem) -> YetiResult<()> {
    let mut path = "/".to_string();
    println!("{} YETI Forensic Explorer v0.2.0. Type 'exit' to return.", "[+]".green());

    loop {
        print!("{} {} > ", "yeti".bold().yellow(), path.cyan());
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let cmd: Vec<&str> = input.trim().split_whitespace().collect();
        
        if cmd.is_empty() { continue; }
        match cmd[0] {
            "ls" => {
                if let Some(node) = vfs.resolve(&path) {
                    for (name, child) in &node.children {
                        let indicator = if child.is_dir { "DIR".blue() } else { "FIL".white() };
                        println!("  {}  {:>10}  {}", indicator, child.size, name);
                    }
                }
            },
            "cd" => {
                if cmd.len() > 1 {
                    if cmd[1] == ".." { path = "/".to_string(); }
                    else { path = format!("{}/{}", path, cmd[1]).replace("//", "/"); }
                }
            },
            "exit" => break,
            _ => println!("Command not found."),
        }
    }
    Ok(())
}
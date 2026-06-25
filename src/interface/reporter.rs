use crate::core::scanner::Candidate;
use colored::Colorize;

pub fn print_summary_table(results: &[Candidate]) {
    println!("\n{:<15} | {:<25} | {:<5}", "Offset", "Signature", "Confidence");
    println!("{}", "-".repeat(55));
    for res in results {
        println!("{:<15x} | {:<25} | {:.2}", res.offset, res.name.green(), res.score);
    }
}
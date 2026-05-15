mod parser;
mod stats;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use parser::{parse_line, ParseOutcome, FailedLogin};

fn main() {
    // 1. Lire l'argument (le fichier)
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <path_to_log_file>");
        process::exit(1);
    }

    let file_path = &args[1];
    
    // 2. Ouvrir le fichier de manière sécurisée
    let file = File::open(file_path).unwrap_or_else(|err| {
        eprintln!("Error opening file {}: {}", file_path, err);
        process::exit(1);
    });

    let reader = BufReader::new(file);
    let mut failed_events: Vec<FailedLogin> = Vec::new();
    let mut total_lines = 0;
    let mut ignored_malformed = 0;

    // 3. Parser ligne par ligne
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(l) => l,
            Err(_) => continue,
        };
        
        total_lines += 1;
        match parse_line(&line) {
            ParseOutcome::Failed(login) => failed_events.push(login),
            ParseOutcome::Ignored | ParseOutcome::Malformed => ignored_malformed += 1,
        }
    }

    // 4. Afficher les résultats
    println!("TP1 Secure Log Analyzer");
    println!("Input file: {}", file_path);
    println!("Summary:");
    println!("- Total lines read: {}", total_lines);
    println!("- Failed login events: {}", failed_events.len());
    println!("- Ignored or malformed lines: {}", ignored_malformed);

    println!("\nTop source IPs:");
    for (i, (ip, count)) in stats::count_by_ip(&failed_events).iter().take(5).enumerate() {
        println!("{}. {} -> {} failed attempts", i + 1, ip, count);
    }

    println!("\nTop targeted users:");
    for (i, (user, count)) in stats::count_by_user(&failed_events).iter().take(5).enumerate() {
        println!("{}. {} -> {} failed attempts", i + 1, user, count);
    }
}
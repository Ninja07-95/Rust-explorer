mod cli;
mod config;
mod search;

use anyhow::Result;
use clap::Parser;
use std::time::Instant;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    let config = config::SearchConfig::from_cli(&cli);
    
    println!("🔍 Moteur de recherche Rust - Lancement...");
    println!("Dossier: {}", config.path);
    println!("Pattern: {}", config.pattern);
    println!("Threads: {}", config.max_threads);
    println!("---");
    
    let start_time = Instant::now();
    let engine = search::SearchEngine::new(config)?;
    let report = engine.search()?;
    
    // Affichage des résultats
    println!("\n📊 RÉSULTATS DE LA RECHERCHE");
    println!("Fichiers analysés: {}", report.total_files_scanned);
    println!("Correspondances trouvées: {}", report.total_matches);
    println!("Durée totale: {:.2?}", report.total_duration);
    
    for result in &report.results {
        println!("\n📁 {}", result.file_path.display());
        for match_item in &result.matches {
            if match_item.line_number > 0 {
                println!("   Ligne {}: '{}'", match_item.line_number, match_item.content);
            } else {
                println!("   Nom du fichier correspondant");
            }
        }
    }
    
    if report.results.is_empty() {
        println!("\n❌ Aucune correspondance trouvée.");
    }
    
    // Benchmark détaillée
    if cli.benchmark {
        println!("\n⚡ PERFORMANCE");
        println!("Fichiers/seconde: {:.2}", report.performance.files_per_second);
        println!("Temps moyen/fichier: {:.2?}", report.performance.average_scan_time);
        println!("Utilisation threads: {:.1}%", report.performance.thread_utilization * 100.0);
    }
    
    // Sauvegarde JSON 
    if cli.benchmark {
        let json = serde_json::to_string_pretty(&report)?;
        std::fs::write("search_report.json", json)?;
        println!("📄 Rapport sauvegardé dans search_report.json");
    }
    
    Ok(())
}
//anass 25/09/2025

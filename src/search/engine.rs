// the main code 26/09/2025

use crate::search::matchers::Matcher;
use crate::search::results::{SearchResult, Match, SearchReport, PerformanceStats};
use crate::config::SearchConfig;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use std::fs;
use indicatif::{ProgressBar, ProgressStyle};

pub struct SearchEngine {
    config: SearchConfig,
    matcher: Arc<dyn Matcher>,
}

impl SearchEngine {
    pub fn new(config: SearchConfig) -> Result<Self, anyhow::Error> {
        let matcher: Arc<dyn Matcher> = if config.use_regex {
            Arc::new(crate::search::matchers::RegexMatcher::new(&config.pattern)?)
        } else {
            Arc::new(crate::search::matchers::SimpleMatcher::new(&config.pattern))
        };
        
        Ok(Self { config, matcher })
    }
    
    pub fn search(&self) -> Result<SearchReport, anyhow::Error> {
        let start_time = Instant::now();
        
        //collecter tous les fichiers
        let files = self.collect_files()?;
        let total_files = files.len();
        
        println!("Recherche dans {} fichiers...", total_files);
        
        // Barre de progression
        let pb = ProgressBar::new(total_files as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap());
        
        // Recherche parallèle avec Rayon
        let results = Arc::new(Mutex::new(Vec::new()));
        let matcher = self.matcher.clone();
        
        rayon::scope(|s| {
            let chunk_size = (files.len() / self.config.max_threads).max(1);
            
            for chunk in files.chunks(chunk_size) {
                let chunk = chunk.to_vec();
                let results = Arc::clone(&results);
                let matcher = matcher.clone();
                let pb = pb.clone();
                
                s.spawn(move |_| {
                    for file_path in chunk {
                        if let Ok(result) = self.search_file(&file_path, &matcher) {
                            if !result.matches.is_empty() {
                                results.lock().unwrap().push(result);
                            }
                        }
                        pb.inc(1);
                    }
                });
            }
        });
        
        pb.finish_with_message("Recherche terminée");
        
        let total_duration = start_time.elapsed();
        let results = Arc::try_unwrap(results).unwrap().into_inner().unwrap();
        
        self.generate_report(results, total_files, total_duration)
    }
    
    fn collect_files(&self) -> Result<Vec<PathBuf>, anyhow::Error> {
        let mut files = Vec::new();
        self.walk_dir(Path::new(&self.config.path), &mut files)?;
        Ok(files)
    }
    
    fn walk_dir(&self, dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), anyhow::Error> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_dir() {
                    self.walk_dir(&path, files)?;
                } else {
                    files.push(path);
                }
            }
        }
        Ok(())
    }
    
    fn search_file(&self, file_path: &Path, matcher: &Arc<dyn Matcher>) -> Result<SearchResult, anyhow::Error> {
        let scan_start = Instant::now();
        
        let content = fs::read_to_string(file_path).unwrap_or_default();
        let matches_data = if self.config.search_content {
            matcher.find_matches(&content)
        } else {
            // Recherche seulement dans le nom du fichier
            let file_name = file_path.file_name().unwrap().to_string_lossy();
            if matcher.matches(&file_name) {
                vec![(0, file_name.to_string(), (0, file_name.len()))]
            } else {
                vec![]
            }
        };
        
        let matches = matches_data.into_iter().map(|(line_num, content, position)| {
            Match { line_number: line_num, content, position }
        }).collect();
        
        Ok(SearchResult {
            file_path: file_path.to_path_buf(),
            matches,
            scan_duration: scan_start.elapsed(),
        })
    }
    
    fn generate_report(&self, results: Vec<SearchResult>, total_files: usize, total_duration: Duration) -> Result<SearchReport, anyhow::Error> {
        let total_matches = results.iter().map(|r| r.matches.len()).sum();
        
        let performance = PerformanceStats {
            files_per_second: total_files as f64 / total_duration.as_secs_f64(),
            average_scan_time: if total_files > 0 {
                Duration::from_secs_f64(total_duration.as_secs_f64() / total_files as f64)
            } else {
                Duration::ZERO
            },
            thread_utilization: (self.config.max_threads as f64).min(total_files as f64) / self.config.max_threads as f64,
        };
        
        Ok(SearchReport {
            total_files_scanned: total_files,
            total_matches,
            total_duration,
            results,
            performance,
        })
    }
}

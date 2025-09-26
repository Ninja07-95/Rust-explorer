// lets make results 
use serde::Serialize;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    pub file_path: PathBuf,
    pub matches: Vec<Match>,
    pub scan_duration: Duration,
}

#[derive(Debug, Clone, Serialize)]
pub struct Match {
    pub line_number: usize,
    pub content: String,
    pub position: (usize, usize),
}

#[derive(Debug, Serialize)]
pub struct SearchReport {
    pub total_files_scanned: usize,
    pub total_matches: usize,
    pub total_duration: Duration,
    pub results: Vec<SearchResult>,
    pub performance: PerformanceStats,
}

#[derive(Debug, Serialize)]
pub struct PerformanceStats {
    pub files_per_second: f64,
    pub average_scan_time: Duration,
    pub thread_utilization: f64,
}

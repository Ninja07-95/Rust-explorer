use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SearchConfig {
    pub path: String,
    pub pattern: String,
    pub search_content: bool,
    pub use_regex: bool,
    pub max_threads: usize,
    pub benchmark: bool,
}

impl SearchConfig {
    pub fn from_cli(cli: &crate::cli::Cli) -> Self {
        Self {
            path: cli.path.clone(),
            pattern: cli.pattern.clone(),
            search_content: cli.content,
            use_regex: cli.regex,
            max_threads: cli.threads,
            benchmark: cli.benchmark,
        }
    }
}

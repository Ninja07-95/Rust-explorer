// les matchers de recherche

use regex::Regex;
use std::path::Path;

pub trait Matcher: Send + Sync {
    fn matches(&self, content: &str) -> bool;
    fn find_matches(&self, content: &str) -> Vec<(usize, String, (usize, usize))>;
}

pub struct SimpleMatcher {
    pattern: String,
}

pub struct RegexMatcher {
    regex: Regex,
}

impl SimpleMatcher {
    pub fn new(pattern: &str) -> Self {
        Self {
            pattern: pattern.to_lowercase(),
        }
    }
}

impl Matcher for SimpleMatcher {
    fn matches(&self, content: &str) -> bool {
        content.to_lowercase().contains(&self.pattern)
    }
    
    fn find_matches(&self, content: &str) -> Vec<(usize, String, (usize, usize))> {
        let mut matches = Vec::new();
        let content_lower = content.to_lowercase();
        let pattern = &self.pattern;
        
        for (line_num, line) in content.lines().enumerate() {
            if line.to_lowercase().contains(pattern) {
                if let Some(pos) = line.to_lowercase().find(pattern) {
                    matches.push((
                        line_num + 1,
//                        line_
                        line.to_string(),
                        (pos, pos + pattern.len())
                    ));
                }
            }
        }
        matches
    }
}

impl RegexMatcher {
    pub fn new(pattern: &str) -> Result<Self, regex::Error> {
        Ok(Self {
            regex: Regex::new(pattern)?,
        })
    }
}

impl Matcher for RegexMatcher {
    fn matches(&self, content: &str) -> bool {
        self.regex.is_match(content)
    }
    
    fn find_matches(&self, content: &str) -> Vec<(usize, String, (usize, usize))> {
        let mut matches = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            for capture in self.regex.find_iter(line) {
                matches.push((
                    line_num + 1,
                    line.to_string(),
                    (capture.start(), capture.end())
                ));
            }
        }
        matches
    }
}

pub mod engine;
pub mod matchers;
pub mod results;

pub use engine::SearchEngine;
pub use results::{SearchReport, SearchResult, Match};
pub use matchers::{Matcher, SimpleMatcher, RegexMatcher};

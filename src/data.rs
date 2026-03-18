use serde::Deserialize;
use std::collections::HashMap;

/// A single accomplishment with tags and impact classification
#[derive(Debug, Deserialize)]
pub struct Accomplishment {
    pub text: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub impact: Option<String>,
}

/// Skill taxonomy entry
#[derive(Debug, Deserialize)]
pub struct Skill {
    pub display: String,
    pub category: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub years: Option<u32>,
    #[serde(default)]
    pub related: Vec<String>,
}

/// A complete experience entry — flattened format for single-file data.toml
#[derive(Debug, Deserialize)]
pub struct Experience {
    pub company: String,
    pub title: String,
    pub start: String,
    pub end: Option<String>,
    pub duration: Option<String>,
    pub location: Option<String>,
    #[serde(rename = "type")]
    pub role_type: Option<String>,
    // Tags (flattened)
    #[serde(default)]
    pub domains: Vec<String>,
    #[serde(default)]
    pub skills: Vec<String>,
    #[serde(default)]
    pub patterns: Vec<String>,
    #[serde(default)]
    pub level: Vec<String>,
    // Phrases (flattened)
    pub headline: Option<String>,
    pub one_liner: Option<String>,
    pub technical: Option<String>,
    // Accomplishments
    #[serde(default)]
    pub accomplishments: Vec<Accomplishment>,
}

impl Experience {
    /// All tags flattened into a single iterator
    pub fn all_tags(&self) -> impl Iterator<Item = &str> {
        self.domains
            .iter()
            .chain(&self.skills)
            .chain(&self.patterns)
            .chain(&self.level)
            .map(|s| s.as_str())
    }
}

/// Top-level data.toml structure
#[derive(Debug, Deserialize)]
pub struct ResumeData {
    #[serde(default)]
    pub skills: HashMap<String, Skill>,
    #[serde(default)]
    pub experience: Vec<Experience>,
}

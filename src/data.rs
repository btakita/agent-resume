use serde::Deserialize;
use std::collections::HashMap;

/// Contact information
#[derive(Debug, Default, Deserialize)]
pub struct Contact {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub location: Option<String>,
    pub linkedin: Option<String>,
    pub github: Option<String>,
    pub website: Option<String>,
}

/// Summary section
#[derive(Debug, Default, Deserialize)]
pub struct Summary {
    pub text: Option<String>,
}

/// Career highlight entry
#[derive(Debug, Deserialize)]
pub struct CareerHighlight {
    pub text: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Education entry
#[derive(Debug, Deserialize)]
pub struct Education {
    pub institution: String,
    pub degree: String,
    pub years: Option<String>,
}

/// Certification entry
#[derive(Debug, Deserialize)]
pub struct Certification {
    pub name: String,
}

/// Language entry
#[derive(Debug, Deserialize)]
pub struct Language {
    pub name: String,
    pub level: Option<String>,
}

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
    pub contact: Contact,
    #[serde(default)]
    pub summary: Summary,
    #[serde(default)]
    pub career_highlights: Vec<CareerHighlight>,
    #[serde(default)]
    pub education: Vec<Education>,
    #[serde(default)]
    pub certifications: Vec<Certification>,
    #[serde(default)]
    pub languages: Vec<Language>,
    #[serde(default)]
    pub skills: HashMap<String, Skill>,
    #[serde(default)]
    pub experience: Vec<Experience>,
}

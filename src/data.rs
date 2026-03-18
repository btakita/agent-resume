use serde::Deserialize;
use std::collections::HashMap;

/// Metadata for an experience entry
#[derive(Debug, Deserialize)]
pub struct ExperienceMeta {
    pub company: String,
    pub title: String,
    pub start: String,
    pub end: Option<String>,
    pub duration: Option<String>,
    pub location: Option<String>,
    #[serde(rename = "type")]
    pub role_type: Option<String>,
}

/// Tags associated with an experience
#[derive(Debug, Default, Deserialize)]
pub struct ExperienceTags {
    #[serde(default)]
    pub domains: Vec<String>,
    #[serde(default)]
    pub skills: Vec<String>,
    #[serde(default)]
    pub patterns: Vec<String>,
    #[serde(default)]
    pub level: Vec<String>,
}

impl ExperienceTags {
    /// All tags flattened into a single iterator
    pub fn all(&self) -> impl Iterator<Item = &str> {
        self.domains
            .iter()
            .chain(&self.skills)
            .chain(&self.patterns)
            .chain(&self.level)
            .map(|s| s.as_str())
    }
}

/// A single accomplishment with tags and impact classification
#[derive(Debug, Deserialize)]
pub struct Accomplishment {
    pub text: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub impact: Option<String>,
}

/// Reusable phrase variants for prose assembly
#[derive(Debug, Default, Deserialize)]
pub struct Phrases {
    pub headline: Option<String>,
    pub one_liner: Option<String>,
    pub technical: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, String>,
}

/// A complete experience entry loaded from TOML
#[derive(Debug, Deserialize)]
pub struct Experience {
    pub meta: ExperienceMeta,
    #[serde(default)]
    pub tags: ExperienceTags,
    #[serde(default)]
    pub accomplishments: Vec<Accomplishment>,
    #[serde(default)]
    pub phrases: Phrases,
}

/// A project entry (shipped product / open source)
#[derive(Debug, Deserialize)]
pub struct Project {
    pub name: String,
    pub url: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub links: HashMap<String, String>,
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

/// Top-level skills file structure
#[derive(Debug, Deserialize)]
pub struct SkillsDb {
    #[serde(default)]
    pub skills: HashMap<String, Skill>,
}

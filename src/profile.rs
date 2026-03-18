use serde::Deserialize;

/// Target matching configuration in a profile
#[derive(Debug, Default, Deserialize)]
pub struct Target {
    #[serde(default)]
    pub match_tags: Vec<String>,
    #[serde(default)]
    pub prioritize: Vec<String>,
    pub max_bullets_per_experience: Option<usize>,
}

/// Highlight configuration (phrase/skill bolding)
#[derive(Debug, Default, Deserialize)]
pub struct Highlight {
    #[serde(default)]
    pub phrases: Vec<String>,
    #[serde(default)]
    pub skills: Vec<String>,
    #[serde(default)]
    pub auto_from_tags: bool,
    #[serde(default)]
    pub extra_phrases: Vec<String>,
}

/// Section filtering
#[derive(Debug, Default, Deserialize)]
pub struct Sections {
    #[serde(default)]
    pub include: Vec<String>,
    #[serde(default)]
    pub experience: Vec<String>,
}

/// A resume profile — controls content selection and emphasis
#[derive(Debug, Default, Deserialize)]
pub struct Profile {
    #[serde(default)]
    pub target: Target,
    #[serde(default)]
    pub highlight: Highlight,
    #[serde(default)]
    pub sections: Sections,
}

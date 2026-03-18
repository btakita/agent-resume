use crate::data::ResumeData;
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Serialize)]
pub struct SearchIndex {
    pub experiences: Vec<SearchExperience>,
    pub skills: Vec<SearchSkill>,
    pub all_tags: Vec<String>,
}

#[derive(Serialize)]
pub struct SearchExperience {
    pub company: String,
    pub title: String,
    pub start: String,
    pub end: Option<String>,
    pub duration: Option<String>,
    pub location: Option<String>,
    pub tags: Vec<String>,
    pub headline: Option<String>,
    pub accomplishments: Vec<SearchAccomplishment>,
}

#[derive(Serialize)]
pub struct SearchAccomplishment {
    pub text: String,
    pub tags: Vec<String>,
    pub impact: Option<String>,
}

#[derive(Serialize)]
pub struct SearchSkill {
    pub name: String,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub years: Option<u32>,
}

/// Build a search index from resume data
pub fn build_search_index(data: &ResumeData) -> SearchIndex {
    let mut all_tags = BTreeSet::new();

    let experiences: Vec<SearchExperience> = data
        .experience
        .iter()
        .map(|exp| {
            let tags: Vec<String> = exp.all_tags().map(|s| s.to_string()).collect();
            for tag in &tags {
                all_tags.insert(tag.clone());
            }
            for acc in &exp.accomplishments {
                for tag in &acc.tags {
                    all_tags.insert(tag.clone());
                }
            }
            SearchExperience {
                company: exp.company.clone(),
                title: exp.title.clone(),
                start: exp.start.clone(),
                end: exp.end.clone(),
                duration: exp.duration.clone(),
                location: exp.location.clone(),
                tags,
                headline: exp.headline.clone(),
                accomplishments: exp
                    .accomplishments
                    .iter()
                    .map(|a| SearchAccomplishment {
                        text: a.text.clone(),
                        tags: a.tags.clone(),
                        impact: a.impact.clone(),
                    })
                    .collect(),
            }
        })
        .collect();

    let skills: Vec<SearchSkill> = data
        .skills
        .values()
        .map(|s| {
            for tag in &s.tags {
                all_tags.insert(tag.clone());
            }
            SearchSkill {
                name: s.display.clone(),
                category: s.category.clone(),
                tags: s.tags.clone(),
                years: s.years,
            }
        })
        .collect();

    SearchIndex {
        experiences,
        skills,
        all_tags: all_tags.into_iter().collect(),
    }
}

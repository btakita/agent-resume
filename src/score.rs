use crate::data::{Accomplishment, Experience};
use crate::profile::Profile;

/// Score an accomplishment against a profile's target tags.
/// Returns a relevance score (higher = more relevant).
pub fn score_accomplishment(accomplishment: &Accomplishment, profile: &Profile) -> usize {
    let match_tags = &profile.target.match_tags;
    if match_tags.is_empty() {
        return 1; // no filtering — everything is equally relevant
    }
    accomplishment
        .tags
        .iter()
        .filter(|tag| {
            match_tags
                .iter()
                .any(|m| m.eq_ignore_ascii_case(tag))
        })
        .count()
}

/// Score an experience entry against a profile's target tags.
/// Uses both experience-level tags and accomplishment tags.
pub fn score_experience(experience: &Experience, profile: &Profile) -> usize {
    let match_tags = &profile.target.match_tags;
    if match_tags.is_empty() {
        return 1;
    }
    let tag_score: usize = experience
        .tags
        .all()
        .filter(|tag| {
            match_tags
                .iter()
                .any(|m| m.eq_ignore_ascii_case(tag))
        })
        .count();
    let accomplishment_score: usize = experience
        .accomplishments
        .iter()
        .map(|a| score_accomplishment(a, profile))
        .sum();
    tag_score + accomplishment_score
}

/// Select top-N accomplishments from an experience, sorted by relevance.
pub fn select_accomplishments<'a>(
    experience: &'a Experience,
    profile: &Profile,
    max: usize,
) -> Vec<&'a Accomplishment> {
    let mut scored: Vec<_> = experience
        .accomplishments
        .iter()
        .map(|a| (score_accomplishment(a, profile), a))
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0));
    scored.into_iter().take(max).map(|(_, a)| a).collect()
}

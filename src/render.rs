use crate::data::Experience;
use crate::profile::Profile;
use crate::score::select_accomplishments;

/// Render a single experience entry as markdown
pub fn render_experience(experience: &Experience, profile: &Profile) -> String {
    let max_bullets = profile.target.max_bullets_per_experience.unwrap_or(usize::MAX);
    let selected = select_accomplishments(experience, profile, max_bullets);

    let mut out = String::new();

    // Header
    out.push_str(&format!("### {} — *{}*\n\n", experience.company, experience.title));

    // Date line
    let end = experience.end.as_deref().unwrap_or("Present");
    let mut date_line = format!("{} – {}", experience.start, end);
    if let Some(dur) = &experience.duration {
        date_line.push_str(&format!(" ({})", dur));
    }
    if let Some(loc) = &experience.location {
        date_line.push_str(&format!(" | {}", loc));
    }
    out.push_str(&format!("<small>{}</small>\n\n", date_line));

    // Accomplishments as bullets
    for accomplishment in &selected {
        out.push_str(&format!("- {}\n", accomplishment.text));
    }
    out.push('\n');

    out
}

/// Render all experiences sorted by relevance score
pub fn render_all_experiences(experiences: &[Experience], profile: &Profile) -> String {
    let mut scored: Vec<_> = experiences
        .iter()
        .map(|e| (crate::score::score_experience(e, profile), e))
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0));

    let mut out = String::new();
    for (_, exp) in &scored {
        out.push_str(&render_experience(exp, profile));
    }
    out
}

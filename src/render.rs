use crate::data::{Experience, ResumeData};
use crate::profile::Profile;
use crate::score::select_accomplishments;
use regex::Regex;

/// Bold specified phrases in text, avoiding already-bolded or linked text.
pub fn bold_phrases(text: &str, phrases: &[String]) -> String {
    let mut result = text.to_string();
    for phrase in phrases {
        if phrase.is_empty() {
            continue;
        }
        // Match phrase when not already inside **...** or [...] or <...>
        let pattern = format!(
            r"(?<!\*\*)(?<!\[)(?<!<)(?i){}(?!\*\*)(?!\])(?!>)",
            regex::escape(phrase)
        );
        if let Ok(re) = Regex::new(&pattern) {
            result = re
                .replace_all(&result, |caps: &regex::Captures| {
                    format!("**{}**", &caps[0])
                })
                .to_string();
        }
    }
    result
}

/// Bold specified skills in a comma-separated skills line.
pub fn bold_skills(text: &str, skills: &[String]) -> String {
    let mut result = text.to_string();
    for skill in skills {
        result = result.replace(skill.as_str(), &format!("**{}**", skill));
    }
    result
}

/// Render the contact header
fn render_contact(data: &ResumeData) -> String {
    let c = &data.contact;
    let name = c.name.as_deref().unwrap_or("Resume");
    let mut out = format!("# {}\n\n", name);

    // First contact line: LinkedIn, Email, GitHub
    let mut line1 = Vec::new();
    if let Some(li) = &c.linkedin {
        line1.push(format!(
            "**LinkedIn:** [{li}](https://{li}/)"
        ));
    }
    if let Some(email) = &c.email {
        line1.push(format!(
            "**Email:** [{email}](mailto:{email})"
        ));
    }
    if let Some(gh) = &c.github {
        line1.push(format!(
            "**GitHub:** [{gh}](https://github.com/{gh})"
        ));
    }
    if !line1.is_empty() {
        out.push_str(&format!(
            "<div style=\"text-align:center\" class=\"contact-line\">\n{}\n</div>\n\n",
            line1.join(" |\n")
        ));
    }

    // Second contact line: Location, Website, Phone
    let mut line2 = Vec::new();
    if let Some(loc) = &c.location {
        line2.push(loc.clone());
    }
    if let Some(web) = &c.website {
        line2.push(format!(
            "**Website:** [{web}](https://{web})"
        ));
    }
    if let Some(phone) = &c.phone {
        let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
        line2.push(format!("**Phone:** [{phone}](tel:+{digits})"));
    }
    if !line2.is_empty() {
        out.push_str(&format!(
            "<div style=\"text-align:center\" class=\"contact-line\">\n{}\n</div>\n\n",
            line2.join(" |\n")
        ));
    }

    out
}

/// Render a single experience entry as markdown
pub fn render_experience(experience: &Experience, profile: &Profile) -> String {
    let max_bullets = profile.target.max_bullets_per_experience.unwrap_or(usize::MAX);
    let selected = select_accomplishments(experience, profile, max_bullets);

    let mut out = String::new();
    out.push_str(&format!(
        "### {} — _{}_\n\n",
        experience.company, experience.title
    ));

    let end = experience.end.as_deref().unwrap_or("Present");
    let mut date_line = format!("{} – {}", experience.start, end);
    if let Some(dur) = &experience.duration {
        date_line.push_str(&format!(" ({})", dur));
    }
    if let Some(loc) = &experience.location {
        date_line.push_str(&format!(" | {}", loc));
    }
    out.push_str(&format!("<small>{}</small>\n\n", date_line));

    for accomplishment in &selected {
        out.push_str(&format!("- {}\n", accomplishment.text));
    }
    out.push('\n');

    out
}

/// Render the full resume as markdown
pub fn render_resume(data: &ResumeData, profile: &Profile) -> String {
    let mut out = String::new();

    // Contact
    out.push_str(&render_contact(data));

    // Summary
    if let Some(summary_text) = &data.summary.text {
        out.push_str("## Summary\n\n");
        out.push_str(summary_text);
        out.push_str("\n\n");
    }

    // Skills
    if !data.skills.is_empty() {
        out.push_str("## Skills\n\n");
        let mut skill_names: Vec<&str> = data.skills.values().map(|s| s.display.as_str()).collect();
        skill_names.sort();
        let skills_line = skill_names.join(", ");
        let skills_line = if !profile.highlight.skills.is_empty() {
            bold_skills(&skills_line, &profile.highlight.skills)
        } else {
            skills_line
        };
        out.push_str(&skills_line);
        out.push_str("\n\n");
    }

    // Career Highlights
    if !data.career_highlights.is_empty() {
        out.push_str("## Career Highlights\n\n");
        for highlight in &data.career_highlights {
            out.push_str(&format!("- {}\n", highlight.text));
        }
        out.push('\n');
    }

    // Experience
    if !data.experience.is_empty() {
        out.push_str("## Experience\n\n");

        // Filter experiences by profile sections.experience if set
        let experiences: Vec<&Experience> = if !profile.sections.experience.is_empty() {
            let mut ordered = Vec::new();
            for name in &profile.sections.experience {
                if let Some(exp) = data.experience.iter().find(|e| {
                    e.company.starts_with(name.as_str()) || e.company.contains(name.as_str())
                }) {
                    ordered.push(exp);
                }
            }
            ordered
        } else {
            data.experience.iter().collect()
        };

        for exp in &experiences {
            out.push_str(&render_experience(exp, profile));
        }
    }

    // Education
    if !data.education.is_empty() {
        out.push_str("## Education\n\n");
        for edu in &data.education {
            out.push_str(&format!("**{}**\n", edu.institution));
            let mut line = edu.degree.clone();
            if let Some(years) = &edu.years {
                line.push_str(&format!(" ({})", years));
            }
            out.push_str(&format!("{}\n\n", line));
        }
    }

    // Certifications
    if !data.certifications.is_empty() {
        out.push_str("## Certifications\n\n");
        for cert in &data.certifications {
            out.push_str(&format!("- {}\n", cert.name));
        }
        out.push('\n');
    }

    // Languages
    if !data.languages.is_empty() {
        out.push_str("## Languages\n\n");
        for lang in &data.languages {
            let mut line = lang.name.clone();
            if let Some(level) = &lang.level {
                line.push_str(&format!(": {}", level));
            }
            out.push_str(&format!("{}\n", line));
        }
        out.push('\n');
    }

    // Apply phrase bolding to the entire output
    if !profile.highlight.phrases.is_empty() {
        out = bold_phrases(&out, &profile.highlight.phrases);
    }

    out
}

/// Render just experiences sorted by relevance score (legacy)
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

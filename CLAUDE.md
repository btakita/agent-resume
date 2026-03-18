# agent-resume

Rust CLI for tag-based resume content assembly. Reads structured `data.toml` + TOML profiles → tailored markdown resumes.

## Architecture

```
data.toml           Single-file resume database (experiences, skills, career highlights, etc.)
profiles/<name>.toml  Job-specific configuration (match_tags, phrase bolding, section filtering)
src/
├── main.rs         CLI: build, render, validate subcommands
├── lib.rs          Module exports
├── data.rs         ResumeData, Experience, Accomplishment, Skill types
├── profile.rs      Profile with Target (match_tags), Highlight, Sections
├── score.rs        Tag-overlap scoring + accomplishment selection
└── render.rs       Full resume markdown assembly + phrase/skill bolding
```

## Commands

```bash
agent-resume build -d data.toml                      # Full resume, no profile
agent-resume build -d data.toml -p profiles/X.toml   # Tailored resume with profile
agent-resume render -d data.toml                     # Experience section only
agent-resume validate -d data.toml                   # Validate TOML parsing
```

## Data Format

`data.toml` contains:
- `[contact]` — name, email, phone, location, linkedin, github, website
- `[summary]` — text
- `[[career_highlights]]` — text + tags
- `[skills.*]` — display, category, tags, years
- `[[experience]]` — company, title, dates, tags, accomplishments, phrases
- `[[education]]`, `[[certifications]]`, `[[languages]]`

## Profile Format

```toml
[target]
match_tags = ["Python", "React"]     # Score experiences by tag overlap
max_bullets_per_experience = 4       # Limit bullets per experience

[highlight]
phrases = ["real-time", "pipeline"]  # Bold these phrases in output
skills = ["Rust", "Python"]          # Bold these in Skills section

[sections]
experience = ["Presence AI", "Open Source"]  # Filter/reorder experiences
```

## Build

```bash
cargo build --release
cargo install --path .
```

## Integration with resume build pipeline

```bash
# agent-resume handles content assembly
agent-resume build -d data.toml -p profiles/default.toml > tailored.md
# Python handles PDF rendering
pandoc tailored.md --to=html > resume.html
weasyprint resume.html resume.pdf --stylesheet resume.css
```

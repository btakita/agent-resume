# agent-resume

Resume generator with tag-based content assembly. Reads structured experience data from a single TOML file and generates tailored markdown resumes based on job-specific profiles.

## Features

- **Structured data** — experiences, skills, accomplishments stored as tagged TOML entries
- **Tag-based scoring** — profile `match_tags` score and rank experiences by relevance
- **Phrase bolding** — automatically bold key phrases and skills in output
- **Profile-driven** — different profiles produce different resumes from the same data
- **Full assembly** — generates complete resume markdown (contact, summary, skills, highlights, experience, education)

## Quick Start

```bash
# Install
cargo install agent-resume

# Validate your data
agent-resume validate -d data.toml

# Build a resume
agent-resume build -d data.toml -p profiles/backend-ai.toml > resume.md
```

## Data Format

All resume content lives in a single `data.toml`:

```toml
[contact]
name = "Your Name"
email = "you@example.com"

[summary]
text = "Your professional summary..."

[[career_highlights]]
text = "**Company** — Achievement description"
tags = ["AI/ML", "leadership"]

[skills.rust]
display = "Rust"
category = "language"
tags = ["systems", "performance"]
years = 5

[[experience]]
company = "Acme Corp"
title = "Senior Engineer"
start = "2020-01"
end = "2024-01"
skills = ["Rust", "Python"]
patterns = ["distributed systems"]

[[experience.accomplishments]]
text = "Built distributed data pipeline processing 1M events/sec"
tags = ["distributed systems", "performance"]
impact = "core-product"
```

## Profiles

Profiles control what gets emphasized and filtered:

```toml
[target]
match_tags = ["Python", "Django", "API"]
max_bullets_per_experience = 4

[highlight]
phrases = ["distributed systems", "real-time"]
skills = ["Python", "Django"]

[sections]
experience = ["Acme Corp", "Open Source"]
```

## License

MIT

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use agent_resume::data::ResumeData;
use agent_resume::profile::Profile;
use agent_resume::render::{render_all_experiences, render_resume};
use agent_resume::search::build_search_index;

#[derive(Parser)]
#[command(name = "agent-resume", about = "Resume generator with tag-based content assembly")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build full resume markdown from data.toml + profile
    Build {
        /// Path to data.toml file
        #[arg(short, long, default_value = "data.toml")]
        data: PathBuf,
        /// Path to profile TOML file
        #[arg(short, long)]
        profile: Option<PathBuf>,
    },
    /// Render just experiences (filtered and scored by profile)
    Render {
        /// Path to data.toml file
        #[arg(short, long, default_value = "data.toml")]
        data: PathBuf,
        /// Path to profile TOML file
        #[arg(short, long)]
        profile: Option<PathBuf>,
    },
    /// Validate data file (check TOML parsing)
    Validate {
        /// Path to data.toml file
        #[arg(short, long, default_value = "data.toml")]
        data: PathBuf,
    },
    /// Generate search index JSON for browser-based search
    SearchIndex {
        /// Path to data.toml file
        #[arg(short, long, default_value = "data.toml")]
        data: PathBuf,
    },
}

fn load_data(path: &PathBuf) -> Result<ResumeData> {
    let content =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let data: ResumeData =
        toml::from_str(&content).with_context(|| format!("parsing {}", path.display()))?;
    Ok(data)
}

fn load_profile(path: &PathBuf) -> Result<Profile> {
    let content =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let profile: Profile =
        toml::from_str(&content).with_context(|| format!("parsing {}", path.display()))?;
    Ok(profile)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build { data, profile } => {
            let resume_data = load_data(&data)?;
            let profile = match profile {
                Some(p) => load_profile(&p)?,
                None => Profile::default(),
            };
            let output = render_resume(&resume_data, &profile);
            print!("{}", output);
        }
        Commands::Render { data, profile } => {
            let resume_data = load_data(&data)?;
            let profile = match profile {
                Some(p) => load_profile(&p)?,
                None => Profile::default(),
            };
            let output = render_all_experiences(&resume_data.experience, &profile);
            print!("{}", output);
        }
        Commands::SearchIndex { data } => {
            let resume_data = load_data(&data)?;
            let index = build_search_index(&resume_data);
            let json = serde_json::to_string_pretty(&index)?;
            println!("{}", json);
        }
        Commands::Validate { data } => {
            let resume_data = load_data(&data)?;
            eprintln!(
                "Validated {} experiences, {} skills, {} career highlights",
                resume_data.experience.len(),
                resume_data.skills.len(),
                resume_data.career_highlights.len()
            );
            for exp in &resume_data.experience {
                eprintln!(
                    "  {} — {} ({} accomplishments)",
                    exp.company,
                    exp.title,
                    exp.accomplishments.len()
                );
            }
        }
    }

    Ok(())
}

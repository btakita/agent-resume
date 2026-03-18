use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use agent_resume::data::Experience;
use agent_resume::profile::Profile;
use agent_resume::render::render_all_experiences;

#[derive(Parser)]
#[command(name = "agent-resume", about = "Resume generator with tag-based content assembly")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Render experiences as markdown, filtered and scored by profile
    Render {
        /// Path to data directory containing experiences/ and skills.toml
        #[arg(short, long, default_value = "data")]
        data_dir: PathBuf,
        /// Path to profile TOML file
        #[arg(short, long)]
        profile: Option<PathBuf>,
    },
    /// Validate data files (check TOML parsing)
    Validate {
        /// Path to data directory
        #[arg(short, long, default_value = "data")]
        data_dir: PathBuf,
    },
}

fn load_experiences(data_dir: &PathBuf) -> Result<Vec<Experience>> {
    let exp_dir = data_dir.join("experiences");
    if !exp_dir.exists() {
        anyhow::bail!("experiences directory not found: {}", exp_dir.display());
    }
    let pattern = exp_dir.join("*.toml").display().to_string();
    let mut experiences = Vec::new();
    for entry in glob::glob(&pattern)? {
        let path = entry?;
        let content =
            std::fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
        let exp: Experience =
            toml::from_str(&content).with_context(|| format!("parsing {}", path.display()))?;
        experiences.push(exp);
    }
    Ok(experiences)
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
        Commands::Render { data_dir, profile } => {
            let experiences = load_experiences(&data_dir)?;
            let profile = match profile {
                Some(p) => load_profile(&p)?,
                None => Profile::default(),
            };
            let output = render_all_experiences(&experiences, &profile);
            print!("{}", output);
        }
        Commands::Validate { data_dir } => {
            let experiences = load_experiences(&data_dir)?;
            eprintln!("Validated {} experience files", experiences.len());
            for exp in &experiences {
                eprintln!(
                    "  {} — {} ({} accomplishments)",
                    exp.meta.company,
                    exp.meta.title,
                    exp.accomplishments.len()
                );
            }
        }
    }

    Ok(())
}

use anyhow::Context;
use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;
use cargo_metadata::MetadataCommand;
use chrono::Utc;
use std::time::Instant;
use tokio::time::{sleep, Duration as TokioDuration};

mod crate_info;
use crate_info::{ApiResponse, CrateMaintenance};

#[derive(Parser, Debug)]
#[command(name = "cargo")]
enum Cargo {
    #[command(name = "maintenance-score", about = "Detect unmaintained and risky Rust dependencies")]
    MaintenanceScore(MaintenanceScore),
}

#[derive(clap::Args, Debug)]
struct MaintenanceScore {
    #[arg(short, long)]
    manifest_path: Option<PathBuf>,
}

fn calculate_score(days_inactive: i64, recent_downloads: u64) -> (u8, String) {
    let inactivity_score = if days_inactive <= 30 {
        80
    } else if days_inactive <= 90 {
        70
    } else if days_inactive <= 180 {
        60
    } else if days_inactive <= 365 {
        40
    } else if days_inactive <= 730 {
        20
    } else {
        0
    };

    let popularity_bonus = if recent_downloads > 50_000_000 {
        20
    } else if recent_downloads > 10_000_000 {
        15
    } else if recent_downloads > 1_000_000 {
        10
    } else if recent_downloads > 100_000 {
        5
    } else {
        0
    };

    let total_score = (inactivity_score + popularity_bonus).min(100);

    let risk = if total_score >= 70 {
        "Low".green()
    } else if total_score >= 40 {
        "Medium".yellow()
    } else {
        "⚠ High".red().bold()
    };

    (total_score, risk.to_string())
}

async fn fetch_crate_data(name: &str) -> anyhow::Result<CrateMaintenance> {
    let client = reqwest::Client::builder()
        .user_agent("cargo-maintenance-score/0.1 (modeleke61@gmail.com)") // use email or github user url if desired
        .build()?;

    let url = format!("https://crates.io/api/v1/crates/{}", name);
    let response = client.get(&url).send().await?;

    if response.status() == 404 {
        return Err(anyhow::anyhow!("Not found"));
    }
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("HTTP {}", response.status()));
    }

    let api_resp: ApiResponse = response.json().await?;

    let last_updated = chrono::DateTime::parse_from_rfc3339(&api_resp.crate_info.updated_at)?
        .with_timezone(&Utc);

    let days_since_update = (Utc::now() - last_updated).num_days();
    let recent_downloads = api_resp.crate_info.recent_downloads.unwrap_or(0);

    let (score, risk_level) = calculate_score(days_since_update, recent_downloads);

    Ok(CrateMaintenance {
        days_since_update,
        recent_downloads,
        max_version: api_resp.crate_info.max_version,
        score,
        risk_level,
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Cargo::MaintenanceScore(args) = Cargo::parse();

    let manifest_path = match args.manifest_path {
        Some(path) => path,
        None => std::env::current_dir()?.join("Cargo.toml"),
    };

    let metadata = MetadataCommand::new()
        .manifest_path(&manifest_path)
        .exec()
        .context("Run in a Rust project directory with Cargo.toml")?;

    let mut crate_names: Vec<String> = metadata
        .packages
        .iter()
        .flat_map(|p| &p.dependencies)
        .map(|dep| dep.name.clone())
        .collect();

    crate_names.sort();
    crate_names.dedup();

    if crate_names.is_empty() {
        println!("{}", "No dependencies found.".yellow());
        return Ok(());
    }

    let total = crate_names.len();
    println!("{} {} unique dependency crates", "Found".bold().blue(), total.to_string().bold().blue());
    println!("Fetching maintenance data...\n");

    let start = Instant::now();
    let mut results = Vec::new();

    //scan ALL crates
    for (i, name) in crate_names.iter().enumerate() {
        print!("{:>4}/{:<4} Fetching {}...", i + 1, total, name.cyan());
        match fetch_crate_data(name).await {
            Ok(data) => {
                println!(" {}", "✔".green());
                results.push((name.clone(), Ok(data)));
            }
            Err(e) => {
                println!(" {}", "✘".red());
                println!("         {}", e.to_string().bright_black());
                results.push((name.clone(), Err(e)));
            }
        }
        sleep(TokioDuration::from_millis(600)).await;
    }

    // Sort by score ascending (riskiest first)
    results.sort_by_key(|(_, r)| r.as_ref().map_or(0, |d| d.score));

    println!("\n{}", "🔍 Maintenance Health Report (Riskiest First)".bold().underline().bright_magenta());
    println!("{:<30} {:>6} {:<12} {:>15} {:>18} {:<12}",
             "Crate", "Score", "Risk", "Days Inactive", "Recent DLs", "Version");

    let mut high = 0;
    let mut medium = 0;
    let mut low = 0;

    for (name, result) in &results {
        match result {
            Ok(data) => {
                if data.score < 40 { high += 1; }
                else if data.score < 70 { medium += 1; }
                else { low += 1; }

                let score_colored = if data.score >= 70 {
                    format!("{:>6}", data.score).bright_green()
                } else if data.score >= 40 {
                    format!("{:>6}", data.score).bright_yellow()
                } else {
                    format!("{:>6}", data.score).bright_red()
                };

                println!("{:<30} {} {:<12} {:>15} {:>18} {:<12}",
                         name.green(),
                         score_colored,
                         data.risk_level,
                         format!("{} days", data.days_since_update).bright_black(),
                         format!("{}", data.recent_downloads).bright_white(),
                         data.max_version.blue());
            }
            Err(_) => {
                println!("{:<30} {:>6} {:<12} {:>15} {:>18} {:<12}",
                         name.red(), "—".bright_black(), "Unknown".bright_black(), "-", "-", "-");
            }
        }
    }

    println!("\n{} Fetched {} crates in {:.1}s", "✔".green(), results.len(), start.elapsed().as_secs_f32());

    println!("\n📊 {} Summary:", "Summary".bold().cyan());
    println!("   {} High risk (potentially unmaintained)", format!("⚠ {}", high).red().bold());
    println!("   {} Medium risk", format!("{} {}", medium, "Medium").yellow());
    println!("   {} Low risk", format!("{} {}", low, "Low").green());

    if high > 0 {
        println!("\n{} Consider reviewing or replacing high-risk crates!", "⚡".bold().yellow());
    }

    Ok(())
}

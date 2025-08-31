use std::{thread, time::Duration};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::get;
use serde::Deserialize;
use clap::Parser;

/// 🌦️ A fancy Rust Weather CLI
#[derive(Parser, Debug)]
#[command(name = "Weather CLI")]
#[command(about = "✨ Get beautiful weather updates from your terminal", long_about = None)]
struct Args {
    /// City name
    city: String,
}

#[derive(Deserialize)]
struct Weather {
    current_condition: Vec<Condition>,
}

#[derive(Deserialize)]
struct Condition {
    temp_C: String,
    weatherDesc: Vec<Desc>,
    humidity: String,
    FeelsLikeC: String,
}

#[derive(Deserialize)]
struct Desc {
    value: String,
}

fn main() {
    let args = Args::parse();
    let city = args.city;

    let url = format!("https://wttr.in/{}?format=j1", city);

    // animated spinner
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message(format!("Summoning weather for {} 🌍", city.bold().bright_cyan()));

    thread::sleep(Duration::from_secs(1));

    // fetch
    let resp = get(&url).unwrap().json::<Weather>().unwrap();
    pb.finish_and_clear();

    let cond = &resp.current_condition[0];
    let temp = &cond.temp_C;
    let feels_like = &cond.FeelsLikeC;
    let desc = &cond.weatherDesc[0].value;
    let humidity = &cond.humidity;

    // fancy banner
    println!(
        "\n{}",
        "═════════════════════════════════════════════".bright_black()
    );
    println!(
        "{} {}",
        "🌤️ Weather Report for".bold().bright_yellow(),
        city.bold().bright_cyan()
    );
    println!(
        "{}",
        "═════════════════════════════════════════════".bright_black()
    );

    // data
    println!("{} {}", "🌡️ Temperature:".bold().bright_white(), format!("{}°C", temp).bright_green());
    println!("{} {}", "🤔 Feels Like:".bold().bright_white(), format!("{}°C", feels_like).bright_blue());
    println!("{} {}", "☁️ Condition:".bold().bright_white(), desc.bright_magenta().italic());
    println!("{} {}", "💧 Humidity:".bold().bright_white(), format!("{}%", humidity).bright_cyan());

    println!(
        "\n{}",
        "═════════════════════════════════════════════".bright_black()
    );
    println!("{}", "✨ Stay awesome & enjoy your day!".bright_yellow());
}


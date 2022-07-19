//! Main executable for the Cobalt web framework.

pub mod tokenizer;
pub mod parser;
pub mod emitter;

use serde_derive::Deserialize;
use std::fs;

/// Holds website configuration information.
#[derive(Deserialize)]
pub struct Config {
    site: Site,
    style: Style,
}

/// Holds general information about the website.
#[derive(Deserialize)]
pub struct Site {
    // Holds the website name
    name: String,
    // Holds optional instructions on how to format the page title
    //
    // Options are "page", "site", "page | site", and "site | page"
    // If not specified, Cobalt defaults to "page"
    title: Option<String>,
}

/// Holds information about the website's CSS style.
#[derive(Deserialize)]
pub struct Style {
    default: String,
    external: Option<Vec<String>>,
}

fn main() {
    // Import and parse the configuration file
    let config: String = match fs::read_to_string("cobalt.toml") {
        Ok(t) => t,
        Err(_) => todo!(),
    };
    let toml: Config = match toml::from_str(&config) {
        Ok(t) => t,
        Err(_) => todo!(),
    };

    
}
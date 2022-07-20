//! Main executable for the Cobalt web framework.

pub mod tokenizer;
pub mod parser;
pub mod emitter;

use std::{
    fs,
    ffi::OsStr,
};

use serde_derive::Deserialize;
use walkdir::WalkDir;

use tokenizer::Tokenizer;
use parser::Parser;
use emitter::Emitter;

/// Holds website configuration information.
#[derive(Clone, Deserialize)]
pub struct Config {
    site: Site,
    style: Style,
}

/// Holds general information about the website.
#[derive(Clone, Deserialize)]
pub struct Site {
    // Holds the website name.
    name: String,

    // Holds optional instructions on how to format the page title.
    //
    // Options are "page", "site", "page | site", and "site | page".
    // If not specified, Cobalt defaults to "page".
    title: Option<String>,

    // Holds the Cobalt source directory.
    // 
    // If not specified, Cobalt defaults to the directory name "cobalt".
    path: Option<String>,
}

/// Holds information about the website's CSS style.
#[derive(Clone, Deserialize)]
pub struct Style {
    default: String,
    external: Option<Vec<String>>,
}

fn main() {
    // Import and parse the configuration file
    let config: String = read("cobalt.toml");
    let toml: Config = match toml::from_str(&config) {
        Ok(t) => t,
        Err(_) => todo!(),
    };

    // Holds a list of source filenames to compile.
    let mut filenames = Vec::new();

    // Recursively walks through the current directory to search for source files.
    let src_directory = match toml.site.path.clone() {
        Some(s) => s,
        None => "cobalt".to_string(),
    };

    for entry in WalkDir::new(&src_directory) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => todo!(),
        };
        if entry.path().extension() == Some(OsStr::new("co")) {
            filenames.push(entry.path().display().to_string());
        }
    }

    for filename in filenames {
        let data = read(&filename);
        let mut tokenizer = Tokenizer::new(data);

        let parser = Parser::new();
        let expressions = parser.parse_all(&mut tokenizer);

        let emitter = Emitter::new(toml.to_owned());
        let output = emitter.emit(expressions);
        dbg!(&output);
    }
}


/// Reads a file to a `String` or throws an error if impossible.
fn read(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(f) => f,
        Err(_) => todo!(),
    }
}
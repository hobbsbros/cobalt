//! Main executable for the Cobalt web framework.

pub mod tokenizer;
pub mod parser;
pub mod emitter;
pub mod error;

use std::{
    fs::{
        self,
        OpenOptions,
    },
    path::Path,
    env,
    io::Write,
    ffi::OsStr,
};

use error::{throw, Error};

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
    // If not specified, Cobalt defaults to the current directory.
    path: Option<String>,
}

/// Holds information about the website's CSS style.
#[derive(Clone, Deserialize)]
pub struct Style {
    default: String,
    external: Option<Vec<String>>,
}

fn main() {
    // Import and parse the configuration file.
    let mut config: String = String::new();
    let mut dir: String = String::new();

    // Recurse through current and parent directories to find configuration file.
    let working_dir = match env::current_dir() {
        Ok(p) => p,
        Err(_) => throw(Error::CouldNotFindToml),
    };

    for path in working_dir.as_path().ancestors() {
        let config_path_buf = path.join("cobalt.toml");
        let config_path = config_path_buf.as_path();
        let string = match config_path.to_path_buf().into_os_string().into_string() {
            Ok(c) => c,
            Err(_) => throw(Error::CouldNotFindToml),
        };
        let parent_dir = match config_path.parent() {
            Some(p) => p,
            None => throw(Error::CouldNotFindToml),
        };
        let dir_string = match parent_dir.to_path_buf().into_os_string().into_string() {
            Ok(c) => c,
            Err(_) => throw(Error::CouldNotFindToml),
        };
        if config_path.is_file() {
            config = string;
            dir = dir_string;
            break;
        }
    }

    let file = read(&config);
    let toml: Config = match toml::from_str(&file) {
        Ok(t) => t,
        Err(_) => throw(Error::CouldNotFindToml),
    };

    // Holds a list of source filenames to compile.
    let mut filenames = Vec::new();

    // Recursively walks through the current directory to search for source files.
    let src_directory = match toml.site.path.clone() {
        Some(s) => s,
        None => dir,
    };

    for entry in WalkDir::new(&src_directory) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => throw(Error::CouldNotReadFile),
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
        let output = emitter.emit(expressions, &Path::new(&src_directory));

        let mut output_filename = filename.clone();
        output_filename.truncate(output_filename.len() - 3);
        output_filename.push_str(".html");

        write(&output_filename, output);
    }
}


/// Reads a file to a `String` or throws an error if impossible.
fn read(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(f) => f,
        Err(_) => throw(Error::CouldNotReadFile),
    }
}


/// Writes a file from a `String` or throws an error if impossible.
fn write(filename: &str, file: String) {
    let mut output = match OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(filename.to_string()) {
            Ok(f) => f,
            Err(_) => throw(Error::CouldNotOpenFile (filename.to_string())),
    };
    match output.write_all(file.as_bytes()) {
        Ok(_) => (),
        Err(_) => throw(Error::CouldNotWriteFile (filename.to_string())),
    };
}
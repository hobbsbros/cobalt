//! Provides an error handler for the Cobalt framework.

use std::process::exit;

use colored::*;

/// Enumerates the types of errors thrown by the Cobalt compiler.
pub enum Error {
    CouldNotReadFile,
    CouldNotFindToml,
    CouldNotWriteFile (String),
    CouldNotOpenFile (String),
    InvalidConfig (String),
    InvalidCtrlSequence (String),
    TooManyHeadings,
    UnexpectedEof,
    ExpectedOpen (String),
    CouldNotParse (String),
    ExpectedHeading (String),
    ExpectedParen (String),
}

use Error::*;

/// Throws errors and halts the program.
pub fn throw(error: Error) -> ! {
    let msg = match error {
        CouldNotReadFile => format!("Could not read input file"),
        CouldNotFindToml => format!("Could not find configuration file 'cobalt.toml'"),
        CouldNotWriteFile (s) => format!("Could not write to file {}", s),
        CouldNotOpenFile (s) => format!("Could not open file {}", s),
        InvalidConfig (s) => format!("Invalid configuration sequence: {}", s),
        InvalidCtrlSequence (s) => format!("Invalid control sequence: {}", s),
        TooManyHeadings => format!("Too many heading symbols '#'"),
        UnexpectedEof => format!("Unexpected end of file when parsing"),
        ExpectedOpen (s) => format!("Expected opening brace '{{', bracket '[', or parenthesis '(', but got {}", s),
        CouldNotParse (s) => format!("Could not parse near token {}", s),
        ExpectedHeading (s) => format!("Expected heading, got token {}", s),
        ExpectedParen (s) => format!("Expected opening parenthesis '(' but got {}", s),
    };

    println!("{}: {}\nCompiler exiting.", "error".red().bold(), msg);

    exit(0);
}
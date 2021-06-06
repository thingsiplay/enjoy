use crate::settings::file;

use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

use atty::Stream;

/// Reads in each line from stdin, if anything is given.
pub(crate) fn list_from_stdin() -> Result<Vec<String>, Box<dyn Error>> {
    let mut list: Vec<String> = vec![];

    if atty::is(Stream::Stdout) && atty::isnt(Stream::Stdin) {
        for line in io::stdin().lock().lines() {
            list.push(line?);
        }
    }

    Ok(list)
}

/// Prints out a non empty path.
pub fn print_path(path: &Option<PathBuf>) {
    let string_path: String = file::to_str(path.as_ref());

    if !string_path.is_empty() {
        println!("{}", string_path);
    }
}

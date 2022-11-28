/// This program is to learn how to handling error with crate `anyhow`
use anyhow::{Context, Result};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    match load_config("Cargo.toml") {
        Ok(config) => println!("cfg: {:#?}", config),
        Err(error) => println!("err: {:#?}", error),
    }

    match load_config("Config.txt") {
        Ok(config) => println!("cfg: {:#?}", config),
        Err(error) => println!("err: {:#?}", error),
    }
}

fn load_config(filename: &str) -> Result<String> {
    let mut file = File::open(filename).with_context(|| format!("Failed to open file: {filename}"))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

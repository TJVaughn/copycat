// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{thread};
use std::fs::File;
use std::io::prelude::*;
mod monitor;
mod ui;

pub const PATH: &str = "./logs/items.txt";
pub const SPLIT_STR: &str = "||**||";

fn main() {
    thread::spawn(|| {
        monitor::monitor();
    });
    ui::start();
}
fn open_file<P: std::convert::AsRef<std::path::Path>>(path: P) -> std::io::Result<String> {
    let mut open_file = File::open(path)?;
    let mut contents = String::new();
    open_file.read_to_string(&mut contents).expect("error opening file");

    Ok(contents)
}

pub fn save_latest(val: String) -> std::io::Result<()> {
    let opened_file = open_file(PATH);

    match opened_file {
        Ok(contents) => {
            let full_data = val.to_string() + SPLIT_STR + &contents;
            let mut file = File::create(PATH)?;
            file.write_all(full_data.as_bytes())?;
        }
        Err(err) => println!("error!: {err}"),
    }

    Ok(())
}

pub fn save_latest_and_remove(val: String)-> std::io::Result<()>  {
    let opened_file = open_file(PATH);

    match opened_file {
        Ok(contents) => {
            let split_contents: Vec<&str> = contents.split(&val).collect();
            let joined = split_contents.join(SPLIT_STR);
            let full_data = val.to_string() + SPLIT_STR + &joined;

            let mut file = File::create(PATH)?;
            file.write_all(full_data.as_bytes())?;
        }
        Err(err) => println!("error!: {err}"),
    }

    Ok(())
}
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::thread;
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
pub fn open_file<P: std::convert::AsRef<std::path::Path>>(path: P) -> std::io::Result<String> {
    let mut open_file = File::open(path)?;
    let mut contents = String::new();
    open_file.read_to_string(&mut contents);

    Ok(contents)
}
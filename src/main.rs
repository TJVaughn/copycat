// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::fs::File;
use std::io::prelude::*;
use std::{io, thread};
mod monitor;
mod ui;

pub const PATH: &str = "./logs/buffered.bin";

fn main() {
    thread::spawn(|| {
        monitor::monitor();
    });
    ui::start();
}

fn dedup_and_clean(strings: Vec<String>) -> io::Result<Vec<String>> {
    let mut cleaned: Vec<String> = Vec::new();
    // remove empty and whitespace
    for x in 0..strings.len() {
        if !strings[x].trim().is_empty() {
            cleaned.push(strings[x].trim().to_string());
        }
    }

    let length = cleaned.len();

    'outer: for i in 0..length {
        for x in 0..length {
            if x != i {
                if cleaned[x] == cleaned[i] {
                    cleaned.remove(x);
                    break 'outer;
                }
            }
        }
    }

    return Ok(cleaned);
}

pub fn write_strings_to_file(file_path: &str, strings: &[String]) -> io::Result<()> {
    let cleaned_strings =
        dedup_and_clean(strings.to_vec()).expect("You done did screwed up my laundry!");

    let mut file = File::create(file_path)?;
    for string in cleaned_strings {
        let len = string.len();
        file.write_all(&(len as u32).to_le_bytes())?;
        file.write_all(string.as_bytes())?;
    }
    Ok(())
}

pub fn read_strings_from_file(file_path: &str) -> io::Result<Vec<String>> {
    let mut byte_file = File::open(file_path).expect("Could not open");
    let mut strings = Vec::new();
    loop {
        let mut len_bytes = [0u8; 4];
        let bytes_read = byte_file.read(&mut len_bytes)?;
        if bytes_read == 0 {
            break;
        }
        let len = u32::from_le_bytes(len_bytes) as usize;
        let mut string_bytes = vec![0u8; len];
        byte_file.read_exact(&mut string_bytes)?;
        let string = String::from_utf8(string_bytes).expect("error getting string");
        strings.push(string);
    }
    Ok(strings)
}

// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use clipboard::x11_clipboard::{Clipboard, X11ClipboardContext};
use clipboard::ClipboardProvider;
use eframe::{egui, Frame};
use egui::Ui;
use std::fs::{self, File};
use std::io::prelude::*;
use std::{io, thread};
extern crate clipboard;

mod edit_ui;
mod monitor;
mod ui;

pub const COPIED_PATH: &str = "./logs/items.bin";
pub const SAVED_PATH: &str = "./logs/saved.bin";
pub const SAVED_LABELS_PATH: &str = "./logs/saved-l.bin";
pub static mut SHOULD_OPEN_EDIT: bool = false;
pub const WIDTH: f32 = 300.0;
pub const HEIGHT: f32 = 700.0;
pub const TEXT_SIZE: f32 = 12.0;
pub const BTN_WIDTH: f32 = 150.0;
pub const SPACER: f32 = 50.0;
pub static mut FETCH_LATEST: bool = false;

fn main() {

    thread::spawn(|| {
        monitor::copy_events();
    });
    monitor::super_event();

    unsafe {
        if SHOULD_OPEN_EDIT {
            edit_ui::start_edit();
        }
    }
}

fn dedup_and_clean(strings: Vec<String>) -> io::Result<Vec<String>> {
    let mut cleaned: Vec<String> = Vec::new();
    // remove empty and whitespace
    for x in 0..strings.len() {
        if !strings[x].trim().is_empty() {
            cleaned.push(strings[x].trim().to_string());
        }
    }

    // let length = cleaned.len();

    // find dup and remove, then stop the loop
    // 'outer: for i in 0..length {
    //     for x in 0..length {
    //         if x != i {
    //             if cleaned[x] == cleaned[i] {
    //                 cleaned.remove(x);
    //                 break 'outer;
    //             }
    //         }
    //     }
    // }

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

fn create_file(path: &str) -> io::Result<File> {
    let file = File::open(path);
    match file {
        Ok(file) => {
            return Ok(file);
        }
        Err(_) => {
            fs::write(path, Vec::new()).expect("Could not write");
            return File::open(path);
        }
    }
}

pub fn read_strings_from_file(file_path: &str) -> io::Result<Vec<String>> {
    let mut byte_file = create_file(file_path).expect("Error creating file");

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

pub fn show_items(
    title: &str,
    ui: &mut Ui,
    mut values: Vec<String>,
    labels: Vec<String>,
    clip: &mut X11ClipboardContext<Clipboard>,
    path: &str,
    frame: &mut Frame,
) {
    const WIDTH: f32 = 300.0;

    ui.menu_button(title, |ui| {
        let mut num_recent_btns = 1;

        if values.len() > 15 {
            num_recent_btns = 2;
        }

        for i in 0..num_recent_btns {
            let mut for_start = 0;
            let mut for_end = values.len();

            if num_recent_btns > 1 {
                if i == 0 {
                    for_end = 15;
                } else if i == 1 {
                    for_start = 15;
                    for_end = values.len();
                }
            }
            // read_strings_from_file, write_strings_to_file,
            let start = (for_start + 1).to_string();
            let end = (for_end).to_string();

            let title = start + "-" + &end;
            ui.set_width(50.0);

            ui.menu_button(title, |ui| {
                ui.set_width(WIDTH - 100.0);

                for x in for_start..for_end {
                    let substring =
                        labels[x].chars().skip(0).take(30).collect::<String>() as String;

                    let mut displayed_entry = (x + 1).to_string() + ": " + &substring;

                    if substring.len() == 30 {
                        displayed_entry = displayed_entry + "...";
                    }

                    if ui.button(displayed_entry).clicked() {
                        clip.set_contents(values[x].trim().to_string())
                            .expect("error setting contents");

                        values.insert(0, values[x].to_string());

                        write_strings_to_file(path, &values).expect("Error writing strings");
                        frame.close();
                    };
                }
            });
        }
    });
}

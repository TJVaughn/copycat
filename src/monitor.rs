extern crate x11_clipboard;
use crate::{open_file, PATH, SPLIT_STR};
use std::fs::File;
use std::io::prelude::*;

use x11_clipboard::Clipboard;

pub fn monitor() {
    let clipboard = Clipboard::new().unwrap();

    loop {
        let val = clipboard.load_wait(
            clipboard.setter.atoms.clipboard,
            clipboard.setter.atoms.string,
            clipboard.setter.atoms.property,
        );
        match val {
            Ok(result) => {
                let copied_value = String::from_utf8(result).unwrap();
                println!("{}", copied_value);
                match save_latest(copied_value) {
                    Ok(_) => {
                        println!("saved")
                    }
                    Err(err) => {
                        println!("error saving file: {err}")
                    }
                }
            }
            Err(err) => {
                println!("war were declared: {err}")
            }
        }
    }
}

fn save_latest(val: String) -> std::io::Result<()> {
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

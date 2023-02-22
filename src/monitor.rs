extern crate x11_clipboard;
use crate::{ save_latest};

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


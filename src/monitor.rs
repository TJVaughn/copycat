extern crate x11_clipboard;
use crate::{read_strings_from_file, PATH, write_strings_to_file };

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
                let mut read_strings = read_strings_from_file(PATH).expect("Error reading file");

                read_strings.insert(0, copied_value);

                write_strings_to_file(PATH, &read_strings).expect("There was an error writing the file");

            }
            Err(err) => {
                println!("war were declared: {err}")
            }
        }
    }
}


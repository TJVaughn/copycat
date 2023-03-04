extern crate x11_clipboard;

use std::{thread, time::Duration};

use crate::{ ui, COPIED_PATH, SHOULD_OPEN_EDIT, edit_ui,read_strings_from_file,write_strings_to_file};

use device_query::{DeviceQuery, DeviceState, Keycode};
use x11_clipboard::Clipboard;

pub fn copy_events() {
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
                let mut read_strings =
                    read_strings_from_file(COPIED_PATH).expect("Error reading file");

                read_strings.insert(0, copied_value);

                write_strings_to_file(COPIED_PATH, &read_strings)
                    .expect("There was an error writing the file");
            }
            Err(err) => {
                println!("war were declared: {err}")
            }
        }
    }
}

pub fn super_event() -> bool {
    let device_state = DeviceState::new();

    loop {
        let keys = device_state.get_keys();

        unsafe {
            if SHOULD_OPEN_EDIT {
                edit_ui::start_edit();
            }
        }

        if keys.contains(&Keycode::Meta)
            && keys.contains(&Keycode::LShift)
            && keys.contains(&Keycode::V)
        {
            println!("open clippy");
            ui::start();
        }
        thread::sleep(Duration::from_millis(100));
    }
}

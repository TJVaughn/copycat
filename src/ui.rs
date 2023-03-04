
use crate::{SAVED_LABELS_PATH, SAVED_PATH, WIDTH, HEIGHT, SPACER, TEXT_SIZE, FETCH_LATEST, read_strings_from_file, show_items, COPIED_PATH, SHOULD_OPEN_EDIT};

use egui::{FontId, Pos2, TextStyle};
extern crate clipboard;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use device_query::{DeviceQuery, DeviceState, MouseState};


const APP_NAME: &str = "CopyCat";


pub fn start() {
    let device_state = DeviceState::new();
    let mouse: MouseState = device_state.get_mouse();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(WIDTH, HEIGHT)),
        resizable: false,
        follow_system_theme: true,
        transparent: true,
        initial_window_pos: {
            Some(Pos2 {
                x: mouse.coords.0 as f32,
                y: mouse.coords.1 as f32,
            })
        },
        ..Default::default()
    };

    match eframe::run_native(
        APP_NAME,
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    ) {
        Ok(value) => return value,
        Err(err) => {
            println!("error: {err}")
        }
    }
}

struct MyApp {
    updates: i32,
    clip: ClipboardContext,
    saved_strings: Vec<String>,
    copied_strings: Vec<String>,
    saved_labels: Vec<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            updates: 0,
            clip: ClipboardProvider::new().unwrap(),
            saved_strings: read_strings_from_file(SAVED_PATH).expect("error reading saved strings"),
            copied_strings: read_strings_from_file(COPIED_PATH)
                .expect("There was an error reading the copied strings"),
            saved_labels: read_strings_from_file(SAVED_LABELS_PATH)
                .expect("Error reading saved strings"),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.updates += 1;

        if self.copied_strings.len() > 30 {
            self.copied_strings.truncate(30);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            unsafe {
                if FETCH_LATEST {
                    println!("fetch latest");
                    self.saved_labels = read_strings_from_file(SAVED_LABELS_PATH)
                        .expect("Error reading saved strings");
                    self.saved_strings =
                        read_strings_from_file(SAVED_PATH).expect("Error reading saved strings");
                        
                    FETCH_LATEST = false;
                }
            }
            let mouse: MouseState = DeviceState::new().get_mouse();

            if mouse.button_pressed[1] && !ctx.is_pointer_over_area() && !ui.ui_contains_pointer() {
                println!("mouse click outside");
                frame.close();
            }

            let mut style = (*ctx.style()).clone();
            style.text_styles = [(
                TextStyle::Button,
                FontId::new(TEXT_SIZE, egui::FontFamily::Proportional),
            )]
            .into();

            ctx.set_style(style);
            show_items(
                "recent",
                ui,
                self.copied_strings.to_owned(),
                self.copied_strings.to_owned(),
                &mut self.clip,
                COPIED_PATH,
                frame,
            );
            ui.add_space(SPACER);
            ui.separator();
            show_items(
                "saved",
                ui,
                self.saved_strings.to_owned(),
                self.saved_labels.to_owned(),
                &mut self.clip,
                SAVED_PATH,
                frame,
            );
            ui.add_space(SPACER + SPACER);
            ui.separator();

            if ui.button("edit saved?").clicked() {
                unsafe {
                    SHOULD_OPEN_EDIT = true;
                }
                frame.close();
            }

            ctx.pointer_interact_pos()
        });
    }
}

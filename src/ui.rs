use std::thread;
use std::time::{Duration, SystemTime};

use crate::{read_strings_from_file, write_strings_to_file, SAVED_LABELS_PATH, SAVED_PATH};
use crate::{COPIED_PATH, SHOULD_OPEN_EDIT};
use clipboard::x11_clipboard::{Clipboard, X11ClipboardContext};
use eframe::{egui, Frame};
use egui::{Context, FontId, Pos2, TextStyle, Ui, RichText};
extern crate clipboard;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use device_query::{DeviceQuery, DeviceState, MouseState};

const APP_NAME: &str = "CopyCat";
const WIDTH: f32 = 300.0;
const HEIGHT: f32 = 700.0;
const TEXT_SIZE: f32 = 12.0;
const BTN_WIDTH: f32 = 150.0;
const SPACER: f32 = 50.0;
static mut FETCH_LATEST: bool = false;

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

pub fn start_edit() {
    let device_state = DeviceState::new();
    let mouse: MouseState = device_state.get_mouse();
    // thread::sleep(Duration::from_secs(1));
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
        "Edit Canvas",
        options,
        Box::new(|_cc| Box::new(EditCanvas::default())),
    ) {
        Ok(value) => return value,
        Err(err) => {
            println!("error: {err}");
        }
    };
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

struct EditCanvas {
    add_item_label: String,
    add_item_value: String,
    show_saved: bool,
    now: SystemTime,
    saved_strings: Vec<String>,
    saved_labels: Vec<String>,
}

impl Default for EditCanvas {
    fn default() -> Self {
        Self {
            add_item_label: "".to_owned(),
            add_item_value: "".to_owned(),
            show_saved: false,
            now: SystemTime::now(),
            saved_strings: read_strings_from_file(SAVED_PATH).expect("error reading saved strings"),
            saved_labels: read_strings_from_file(SAVED_LABELS_PATH)
                .expect("Error reading saved strings"),
        }
    }
}

impl eframe::App for EditCanvas {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mouse: MouseState = DeviceState::new().get_mouse();

            if mouse.button_pressed[1] && !ctx.is_pointer_over_area() && !ui.ui_contains_pointer() {
                println!("mouse click outside");
                frame.close();
            }
            unsafe {
                SHOULD_OPEN_EDIT = false;
            }
            ui.heading("Editor Canvas");
            show_editable(
                ui,
                self.saved_strings.to_owned(),
                self.saved_labels.to_owned(),
                self,
                ctx,
                // frame,
            );
            ctx.accesskit_placeholder_tree_update();
        });
    }
}
fn show_editable(
    ui: &mut Ui,
    mut saved_strings: Vec<String>,
    mut saved_labels: Vec<String>,
    edit_data: &mut EditCanvas,
    ctx: &Context,
    // frame: &mut Frame,
) {
    // ui.menu_button("Edit/Add Items", |ui| {
        // show_items("saved", ui, saved_strings, saved_labels, clip, path, frame);
        ui.set_width(WIDTH - WIDTH / 2.0);

        if edit_data.show_saved && edit_data.now.elapsed().expect("now error").as_secs() < 3 {
            ui.label("Saved!").highlight();
        }

        ui.label("Add item");

        ui.label("Item Label");
        ui.text_edit_singleline(&mut edit_data.add_item_label);

        ui.label("Item Value");
        ui.text_edit_multiline(&mut edit_data.add_item_value);

        if ui.button(RichText::new("save")).clicked() {
            ui.set_width(BTN_WIDTH);

            println!(
                "saving: {}: {}",
                edit_data.add_item_label, edit_data.add_item_value
            );
            saved_labels.push(edit_data.add_item_label.to_owned());
            saved_strings.push(edit_data.add_item_value.to_owned());
            write_strings_to_file(SAVED_PATH, &saved_strings).expect("error writing values");
            write_strings_to_file(SAVED_LABELS_PATH, &saved_labels).expect("error writing labels");
            edit_data.add_item_value = "".to_owned();
            edit_data.add_item_label = "".to_owned();
            edit_data.show_saved = true;
            edit_data.now = SystemTime::now();
            thread::sleep(Duration::from_millis(200));
            edit_data.saved_labels =
                read_strings_from_file(SAVED_LABELS_PATH).expect("error reading labels");
            edit_data.saved_strings =
                read_strings_from_file(SAVED_PATH).expect("error reading values");
            ctx.request_repaint();
            unsafe {
                FETCH_LATEST = true;
            }
        }
    // });
}

fn show_items(
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

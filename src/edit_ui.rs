use std::thread;
use std::time::{Duration, SystemTime};

use crate::{
    read_strings_from_file, write_strings_to_file, BTN_WIDTH, FETCH_LATEST, HEIGHT,
    SAVED_LABELS_PATH, SAVED_PATH, SHOULD_OPEN_EDIT, WIDTH,
};
use eframe::egui;
use egui::{Context, Pos2, RichText, Ui};
extern crate clipboard;

use device_query::{DeviceQuery, DeviceState, MouseState};

const EDIT_WIDTH: f32 = 400.0;

pub fn start_edit() {
    let device_state = DeviceState::new();
    let mouse: MouseState = device_state.get_mouse();
    // thread::sleep(Duration::from_secs(1));
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(EDIT_WIDTH, HEIGHT)),
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
    fn on_close_event(&mut self) -> bool  {
        unsafe {
            SHOULD_OPEN_EDIT = false;
        }
        return true;
    }
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            egui::ScrollArea::vertical().show(ui, |ui| {
                show_saved(ui, self);

                show_add_ui(
                    ui,
                    self.saved_strings.to_owned(),
                    self.saved_labels.to_owned(),
                    self,
                    ctx,
                );
                ctx.accesskit_placeholder_tree_update();
            });
        });
    }
}

fn show_saved(ui: &mut Ui, edit_data: &mut EditCanvas) {
    ui.heading("Saved Values");

    'main: for i in 0..edit_data.saved_strings.len() {
        let hori = ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            if ui.button(edit_data.saved_labels[i].to_string()).clicked() {
                edit_data.add_item_label = edit_data.saved_labels[i].to_string();
                edit_data.add_item_value = edit_data.saved_strings[i].to_string();
            }
            if ui.button("X").clicked() {
                edit_data.saved_labels.remove(i);
                edit_data.saved_strings.remove(i);
                return true;
            }
            return false;
        });

        if hori.inner {
            write_strings_to_file(SAVED_LABELS_PATH, &edit_data.saved_labels)
                .expect("error writing labels");
            write_strings_to_file(SAVED_PATH, &edit_data.saved_strings)
                .expect("error writing strings");
            break 'main;
        }
    }

}

fn show_add_ui(
    ui: &mut Ui,
    mut saved_strings: Vec<String>,
    mut saved_labels: Vec<String>,
    edit_data: &mut EditCanvas,
    ctx: &Context,
) {

    ui.set_width(WIDTH);

    if edit_data.show_saved && edit_data.now.elapsed().expect("now error").as_secs() < 3 {
        ui.label("Saved!").highlight();
    }
    ui.with_layout(
        egui::Layout::top_down_justified(egui::Align::Center),
        |ui| {
            ui.heading("Add/Edit item");

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
                write_strings_to_file(SAVED_LABELS_PATH, &saved_labels)
                    .expect("error writing labels");
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
        },
    );
}

use crate::save_latest_and_remove;
use crate::{PATH, SPLIT_STR};
use eframe::egui;
use egui::{FontId, Pos2, TextStyle};
use std::fs;
extern crate clipboard;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use device_query::{DeviceQuery, DeviceState, MouseState};

const APP_NAME: &str = "CopyCat";
const WIDTH: f32 = 300.0;

pub fn start() {
    let device_state = DeviceState::new();
    let mouse: MouseState = device_state.get_mouse();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(WIDTH, 400.0)),
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
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            updates: 0,
            clip: ClipboardProvider::new().unwrap(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        println!("results updated {}", self.updates.to_string());
        self.updates += 1;

        let results = fs::read_to_string(PATH).expect("There was an error opening the file");

        let entries = results.split(SPLIT_STR).filter(|x| !x.trim().is_empty());

        let mut entries_vector: Vec<&str> = entries.collect();
        if entries_vector.len() > 30 {
            entries_vector.truncate(30);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut style = (*ctx.style()).clone();
            style.text_styles = [(
                TextStyle::Button,
                FontId::new(10.0, egui::FontFamily::Proportional),
            )]
            .into();

            ctx.set_style(style);
            ui.menu_button("Recent", |ui| {
                let mut num_recent_btns = 1;

                if entries_vector.len() > 15 {
                    num_recent_btns = 2;
                }

                for i in 0..num_recent_btns {

                    let mut for_start = 0;
                    let mut for_end = entries_vector.len();

                    if num_recent_btns > 1 {
                        if i == 0 {
                            for_end = 15;
                        } else if i == 1 {
                            for_start = 15;
                            for_end = entries_vector.len();
                        }
                    } 

                    let start = (for_start + 1).to_string();
                    let end = (for_end).to_string();

                    let title = start + "-" + &end;
                    ui.set_width(50.0);

                    ui.menu_button(title, |ui| {
                        ui.set_width(WIDTH - 100.0);

                        for x in for_start..for_end {
                            let substring = entries_vector[x]
                                .chars()
                                .skip(0)
                                .take(25)
                                .collect::<String>()
                                as String;

                            let displayed_entry = (x + 1).to_string() + ": " + &substring;
                            let entry = entries_vector[x];
                            if ui.button(displayed_entry).clicked() {
                                self.clip
                                    .set_contents(entry.trim().to_string())
                                    .expect("error setting contents");

                                entries_vector[0] = entry;
                                save_latest_and_remove(entry.to_string()).expect("could not save");
                            };
                        }
                    });
                }
            });
            

            ui.add_space(50.0);
            ui.separator();

            ui.menu_button("Saved", |ui| {
                ui.set_width(50.0);

                ui.menu_button("first saved folder", |ui| {
                    if ui.button("username").clicked() {
                        println!("do things");
                    }
                });
            });
        });
    }
}

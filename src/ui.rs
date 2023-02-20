
use crate::{open_file, PATH, SPLIT_STR};
use eframe::egui;

const APP_NAME: &str = "LazyClip";

pub fn start() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 800.0)),
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
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            updates: 0,
        }
    }
}

fn get_clipboard_results() -> String {
    let opened = open_file(PATH);

    match opened {
        Ok(data) => data,
        Err(err) => ("error: ".to_owned() + &err.to_string()).to_string(),
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let results = get_clipboard_results();
        println!("results updated {}", self.updates.to_string());
        self.updates += 1;

        let entries = results.split(SPLIT_STR);

        egui::CentralPanel::default().show(ctx, |ui| {
            // let mut results = &self.results.to_owned();

            // if ui.button("refresh").clicked() {
            //     latest = get_clipboard_results();
            //     results = latest;
            // }
            // ui.heading(APP_NAME);
            // for i in 0..3 {
                // let start = (i + 1).to_string();
                // let end = ((i + 1) * 10).to_string();

                // let title = start + "-" + &end;

                // ui.menu_button(title, |ui| {

                    for entry in entries {
                        
                        if ui.button(entry).clicked() {
                            ui.close_menu();
                        };
                    }
                // });
            // }
        });
    }
}

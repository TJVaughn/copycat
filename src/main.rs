// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
const APP_NAME: &str = "LazyClip";

extern crate clipboard;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    // tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

fn get_latest()-> String{
    let mut clip: ClipboardContext = ClipboardProvider::new().unwrap();
    // println!("{:?}", ctx.get_contents());
    match clip.get_contents() {
        Ok(value) => {
            println!("Last item copied: {value}");
            return value;
        }
        Err(_) => {
            println!("THERE WERE AN ERROR");
            return "error".to_owned();
        }
    }

}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading(APP_NAME);
            for i in 0..3 {
                let start = (i + 1).to_string();
                let end = ((i + 1) * 10).to_string();

                let title = start + "-" + &end;

                ui.menu_button(title, |ui| {
                    if ui.button(get_latest()).clicked() {

                        ui.close_menu();
                    }
                });
            }
            
        });
    }
}
use clipboard::ClipboardProvider;
use eframe::{App, egui::{CentralPanel, self}};

pub struct MyText {
    pub text: String, // The text to be displayed 
    pub auto_copied: bool, // Whether the text should be copied to the clipboard automatically 
    pub auto_emojify: bool, // Whether the text should be emojified automatically 
}

impl App for MyText {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Quick Emoji");
            ui.separator(); 
            let r = ui.add(eframe::egui::TextEdit::singleline(&mut self.text));
            let mut has_clicked = false; 
            if self.auto_emojify {
                if r.changed() {
                    has_clicked = true; 
                }
            }
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.auto_copied, "Auto copy");
                ui.checkbox(&mut self.auto_emojify, "Auto emojify"); 
            }); 
            if ui.button("Emojify").clicked() {
                has_clicked = true; 
            }
            if has_clicked {
                self.text = emojic::text::parse_text(&self.text); 
                if self.auto_copied {
                    let mut clipboard = clipboard::ClipboardContext::new().unwrap(); 
                    clipboard.set_contents(self.text.clone()).unwrap(); 
                }
            }
        }); 
    }
}

fn main() {
    println!("Hello, world!");
    let native_options : eframe::NativeOptions = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(200.0, 200.0)), 
        ..eframe::NativeOptions::default() 
    }; 
    eframe::run_native("Cutie Emoji", native_options, Box::new( 
        |_| Box::new( MyText {
            text: String::new(), 
            auto_copied: false, 
            auto_emojify: true, 
        }  )
    ) ).unwrap(); 
}

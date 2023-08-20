#![warn(clippy::all, rust_2018_idioms)]

use eframe::egui;

mod api;
mod app;
mod character_gen;
mod model;
mod todos;
mod window_manager;
pub use app::TemplateApp;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait Window {
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}

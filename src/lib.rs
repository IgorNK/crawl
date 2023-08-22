#![warn(clippy::all, rust_2018_idioms)]

use eframe::egui;
use arc_swap::ArcSwap;
use lazy_static::lazy_static;

mod api;
mod app;
mod character_gen;
mod pointbuy_component;
mod raceselect_component;
mod classselect_component;
mod mock_db;
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

lazy_static! {
  static ref STORE: ArcSwap<&'static SystemData> = {
    ArcSwap::from(Arc::new(mock_db::mock_db()))
  };
}

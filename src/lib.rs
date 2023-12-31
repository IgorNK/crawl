#![warn(clippy::all, rust_2018_idioms)]
#[macro_use]
use eframe::egui;
use arc_swap::ArcSwap;
use downcast_rs::{impl_downcast, Downcast};
use lazy_static::lazy_static;
use model::SystemData;
use std::sync::Arc;

mod api;
mod app;
mod character_gen;
mod chat_component;
mod classselect_component;
mod draggable;
mod imageurlprompt_component;
mod mock_db;
mod model;
mod node;
mod node_image;
mod pointbuy_component;
mod raceselect_component;
mod todos;
mod utils;
mod window_manager;
mod zoom_pan;
mod zoom_pan_container;
pub use app::TemplateApp;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context);
}

pub trait Window: Downcast {
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}

impl_downcast!(Window);

lazy_static! {
    static ref STORE: ArcSwap<SystemData> = { ArcSwap::from(Arc::new(mock_db::mock_db())) };
}

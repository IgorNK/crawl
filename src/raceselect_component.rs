use crate::model::{Attributes, Race, SystemData};
use arc_swap::ArcSwap;
use eframe::egui::{self, Align, Layout};
use std::sync::Arc;

pub struct RaceSelectComponent {
    db: Option<Arc<SystemData>>,
}

impl Default for RaceSelectComponent {
    fn default() -> Self {
        RaceSelectComponent { db: None }
    }
}

impl crate::View for RaceSelectComponent {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let Self { db } = self;

        egui::Frame::none().show(ui, |ui| {
            ui.label("Race: ");
            if let Some(db) = db {
                if let Some(races) = &db.races {
                    let _: Vec<_> = races
                        .iter()
                        .map(|race| {
                            ui.button(&race.name);
                        })
                        .collect();
                };
            };
        });
    }
}

impl RaceSelectComponent {
    pub fn with_db(&mut self, db: &Arc<SystemData>) -> &mut RaceSelectComponent {
        self.db = Some(db.clone());
        self
    }
}

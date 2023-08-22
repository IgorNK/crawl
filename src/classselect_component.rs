use crate::model::{Attributes, Class, SystemData};
use arc_swap::ArcSwap;
use eframe::egui::{self, Align, Layout};
use std::sync::Arc;

pub struct ClassSelectComponent {
    db: Option<Arc<SystemData>>,
}

impl Default for ClassSelectComponent {
    fn default() -> Self {
        ClassSelectComponent { db: None }
    }
}

impl crate::View for ClassSelectComponent {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let Self { db } = self;

        egui::Frame::none().show(ui, |ui| {
            ui.label("Class: ");
            if let Some(db) = db {
                if let Some(classes) = &db.classes {
                    let _: Vec<_> = classes
                        .iter()
                        .map(|class| {
                            ui.button(&class.name);
                        })
                        .collect();
                };
            };
        });
    }
}

impl ClassSelectComponent {
    pub fn with_db(&mut self, db: &Arc<SystemData>) -> &mut ClassSelectComponent {
        self.db = Some(db.clone());
        self
    }
}

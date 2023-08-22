use crate::model::{SystemData, Class, Attributes};
use eframe::egui::{self, Align, Layout};
use arc_swap::ArcSwap;
use std::sync::Arc;

pub struct ClassSelectComponent {
    db: Option<ArcSwap<Arc<SystemData>>>,
}

impl Default for ClassSelectComponent {
    fn default() -> Self {
        ClassSelectComponent {
            db: None,
        }
    }
}

impl crate::View for ClassSelectComponent {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let Self {
            db,
        } = self;
      
            ui.label("Class: ");
            if let Some(db) = db {
              if let Some(classes) = &db.classes {
                let _: Vec<_> = classes.iter().map(|class| {
                  ui.button(&class.name);
                }).collect();
              };
            };

    }
}

impl ClassSelectComponent {
  pub fn with_db (&mut self, db: ArcSwap<Arc<SystemData>>) -> &mut ClassSelectComponent {
    self.db = Some(db);
    self
  }
}
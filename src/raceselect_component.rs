use crate::model::{SystemData, Race, Attributes};
use eframe::egui::{self, Align, Layout};
use arc_swap::ArcSwap;
use std::sync::Arc;

pub struct RaceSelectComponent {
    db: Option<ArcSwap<Arc<SystemData>>>
}

impl Default for RaceSelectComponent {
    fn default() -> Self {
        RaceSelectComponent {
            db: None,
        }
    }
}

impl crate::View for RaceSelectComponent {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let Self {
            db
        } = self;
      
            ui.label("Race: ");
            if let Some(db) = db {
              if let Some(races) = &**db.races {
                let _: Vec<_> = races.iter().map(|race| {
                  ui.button(&race.name);
                }).collect();
              };
            };

    }
}

impl RaceSelectComponent {
  pub fn with_db (&mut self, db: ArcSwap<Arc<SystemData>>) -> &mut RaceSelectComponent {
    self.db = Some(db);
    self
  }
}
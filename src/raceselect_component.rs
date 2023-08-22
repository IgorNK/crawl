use crate::model::{SystemData, Race, Attributes};
use eframe::egui::{self, Align, Layout};

pub struct RaceSelectComponent {
    db: Option<&SystemData>
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
              if let Some(races) = db.races {
                let _ = races.map(|race| {
                  ui.button(race.name);
                });
              };
            };

    }
}

impl RaceSelectComponent {
  pub fn with_db(mut self, db: &SystemData) -> Self {
    self.db = Some(db);
    self
  }
}
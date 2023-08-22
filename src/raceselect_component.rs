use crate::model::{SystemData, Race, Attributes};
use eframe::egui::{self, Align, Layout};

pub struct RaceSelectComponent<'a> {
    db: Option<&'a SystemData>
}

impl Default for RaceSelectComponent<'_> {
    fn default() -> Self {
        RaceSelectComponent {
            db: None,
        }
    }
}

impl crate::View for RaceSelectComponent<'_> {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let Self {
            db
        } = self;
      
            ui.label("Race: ");
            if let Some(db) = db {
              if let Some(races) = &db.races {
                let _: Vec<_> = races.iter().map(|race| {
                  ui.button(&race.name);
                }).collect();
              };
            };

    }
}

impl<'a> RaceSelectComponent<'a> {
  pub fn with_db (mut self, db: &'a SystemData) -> Self {
    self.db = Some(db);
    self
  }
}
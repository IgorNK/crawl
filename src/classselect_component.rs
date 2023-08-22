use crate::model::{SystemData, Class, Attributes};
use eframe::egui::{self, Align, Layout};

pub struct ClassSelectComponent<'a> {
    db: Option<&'a SystemData>,
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
              if let Some(classes) = db.classes {
                let _: Vec<_> = classes.iter().map(|class| {
                  ui.button(class.name);
                }).collect();
              };
            };

    }
}

impl ClassSelectComponent<'a> {
  pub fn with_db(mut self, db: &'a SystemData) -> Self {
    self.db = Some(db);
    self
  }
}
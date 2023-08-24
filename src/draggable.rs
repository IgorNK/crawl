use eframe::egui;

pub struct Draggable {
  name: String,
  id: egui::Id,  
  rect: egui::Rect,
}

impl Draggable {
  pub fn new(name: &str) -> Self {
    Self { 
      name: name.to_string(),
      id: egui::Id::new(name),
      rect: egui::Rect::from_min_size(
        egui::Pos2{ x: 50f32, y: 50f32 },
        egui::Vec2 { x: 50f32, y: 50f32 }
      ),
    }
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn id(&self) -> &egui::Id {
    &self.id
  }

  pub fn rect(&self) -> &egui::Rect {
    &self.rect
  }

  pub fn translate(&mut self, amnt: egui::Vec2) {
    self.rect.translate(amnt);
  }

  pub fn set_center(&mut self, center: egui::Pos2) {
    self.rect.set_center(center);
  }
}

impl crate::View for Draggable {
    fn ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {        
        ui.put(self.rect().clone(), egui::widgets::Label::new(self.name()));
    }
}
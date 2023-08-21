use crate::model::{PlayerCharacter, Attributes};
use eframe::egui::{self, Layout, Align};

pub struct CharacterGenComponent {
  attr: Attributes,
  attr_add: Attributes,
  points: u32,
}

impl Default for CharacterGenComponent {
    fn default() -> Self {
        dbg!("Call to character component default creation");
        CharacterGenComponent {
          attr: Attributes::new(10),
          attr_add: Attributes::new(0),
          points: 27,
        }
    }
}

impl crate::Window for CharacterGenComponent {
    fn name(&self) -> &'static str {
        "New Character"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        use crate::View as _;
        egui::Window::new(self.name())
            .open(open)
            .default_height(500.0)
            .show(ctx, |ui| self.ui(ui));
    }
}

impl crate::View for CharacterGenComponent {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let Self {attr, attr_add, points} = self;

        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                  ui.label("Strength");
                  ui.label((attr.str + attr_add.str).to_string());
                  if ui.button("-").clicked() {
                    adjust_count(&attr.str, &mut attr_add.str, &mut points, 1);
                  };
                  if ui.button("+").clicked() {
                    adjust_count(&attr.str, &mut attr_add.str, &mut points, -1);
                  };
                });
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                  ui.label("Dexterity");
                  ui.label((attr.dex + attr_add.dex).to_string());
                  if ui.button("-").clicked() {};
                  if ui.button("+").clicked() {};
                });
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                  ui.label("Constitution");
                  ui.label((attr.con + attr_add.con).to_string());
                  if ui.button("-").clicked() {};
                  if ui.button("+").clicked() {};
                });
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                  ui.label("Intelligence");
                  ui.label((attr.int + attr_add.int).to_string());
                  if ui.button("-").clicked() {};
                  if ui.button("+").clicked() {};
                });
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                  ui.label("Wisdom");
                  ui.label((attr.wis + attr_add.wis).to_string());
                  if ui.button("-").clicked() {};
                  if ui.button("+").clicked() {};
                });
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                  ui.label("Charisma");
                  ui.label((attr.cha + attr_add.cha).to_string());
                  if ui.button("-").clicked() {};
                  if ui.button("+").clicked() {};
                });
            });
    }
}

fn can_add(attr: &i8, attr_add: &i8, points: &u32) -> bool {
  return (*attr + *attr_add + 1) < 20 && *points > 0
}

fn can_subtract(attr_add: &i8, points: &u32) -> bool {
  return (*attr_add - 1) > 0 && *points < 27
}

fn adjust_count(attr: &i8, attr_add: &mut i8, points: &mut u32, amount: i8) {
    if (*attr + *attr_add + amount) < 20 && (*attr_add + amount) > 0 {
      *attr_add += amount;
      *points -= amount;
    }
}

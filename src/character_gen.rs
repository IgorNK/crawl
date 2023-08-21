use crate::model::{Attributes, PlayerCharacter};
use eframe::egui::{self, Align, Layout};

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
        let Self {
            ref mut attr,
            ref mut attr_add,
            ref mut points,
        } = self;

        let fields = [
            ("Strength", &mut attr.str, &mut attr_add.str),
            ("Dexterity", &mut attr.dex, &mut attr_add.dex),
            ("Constitution", &mut attr.con, &mut attr_add.con),
            ("Intelligence", &mut attr.int, &mut attr_add.int),
            ("Wisdom", &mut attr.wis, &mut attr_add.wis),
            ("Charisma", &mut attr.cha, &mut attr_add.cha),
        ];

        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                let _ = fields.map(|field| {
                    add_field(ui, field, points);
                });
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                    ui.label("Points left: ");
                    ui.label(points.to_string());
                });
            });
    }
}

fn add_field(
    ui: &mut egui::Ui,
    (label, attr, add_attr): (&str, &mut i8, &mut i8),
    points: &mut u32,
) {
    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
        ui.label(label);
        ui.label((*attr + *add_attr).to_string());
        ui.add_enabled_ui(can_subtract(add_attr, points), |ui| {
            if ui.button("-").clicked() {
                adjust_count(add_attr, points, -1);
            };
        });
        ui.add_enabled_ui(can_add(attr, add_attr, points), |ui| {
            if ui.button("+").clicked() {
                adjust_count(add_attr, points, 1);
            };
        });
        ui.label(add_attr.to_string());
    });
}

fn can_add(attr: &i8, attr_add: &i8, points: &u32) -> bool {
    return (*attr + *attr_add + 1) < 20 && *points > 0;
}

fn can_subtract(attr_add: &i8, points: &u32) -> bool {
    return (*attr_add - 1) >= 0 && *points < 27;
}

fn adjust_count(attr_add: &mut i8, points: &mut u32, amount: i8) {
    *attr_add += amount;
    *points = (*points as i8 - amount) as u32;
}

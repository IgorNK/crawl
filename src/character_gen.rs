use crate::model::PlayerCharacter;
use eframe::egui;

pub struct CharacterGenComponent {}

impl Default for CharacterGenComponent {
    fn default() -> Self {
        dbg!("Call to character component default creation");
        CharacterGenComponent {}
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
        let Self {} = self;

        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                ui.label("Strength");
                ui.label("Dexterity");
                ui.label("Constitution");
                ui.label("Intelligence");
                ui.label("Wisdom");
                ui.label("Charisma");
            });
    }
}

use crate::model::{Attributes, PlayerCharacter, SystemData};
use crate::pointbuy_component::PointBuyComponent;
use crate::raceselect_component::RaceSelectComponent;
use crate::classselect_component::ClassSelectComponent;
use eframe::egui::{self, Align, Layout};
use arc_swap::ArcSwap;
use std::sync::Arc;

pub struct CharacterGenComponent {
  point_buy: PointBuyComponent,
  race_select: RaceSelectComponent,
  class_select: ClassSelectComponent,
}

impl Default for CharacterGenComponent {
    fn default() -> Self {
      CharacterGenComponent {
        point_buy: PointBuyComponent::default(),
        race_select: RaceSelectComponent::default(),
        class_select: ClassSelectComponent::default(),
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
          ref mut point_buy,
          ref mut race_select,
          ref mut class_select,
        } = self;
        let db: ArcSwap<Arc<SystemData>> = crate::STORE.load();
        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {

          ui.button("<");

          egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                race_select.with_db(db).ui(ui);
            });

          egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                point_buy.ui(ui);
            });

          egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                class_select.with_db(db).ui(ui);
            });

          ui.button(">");
          
        });
    }
}

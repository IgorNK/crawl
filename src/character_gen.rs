use crate::classselect_component::ClassSelectComponent;
use crate::model::{Attributes, PlayerCharacter, SystemData};
use crate::pointbuy_component::PointBuyComponent;
use crate::raceselect_component::RaceSelectComponent;
use arc_swap::ArcSwap;
use eframe::egui::{self, Align, Layout};
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
        let db: arc_swap::Guard<Arc<SystemData>> = crate::STORE.load();

        ui.columns(3, |columns| {
            egui::ScrollArea::vertical()
                .id_source("race_select")
                .show(&mut columns[0], |ui| {
                    race_select.with_db(&*db).ui(ui);
                });
            egui::ScrollArea::vertical()
                .id_source("point_buy")
                .show(&mut columns[1], |ui| {
                    point_buy.ui(ui);
                });
            egui::ScrollArea::vertical()
                .id_source("class_select")
                .show(&mut columns[2], |ui| {
                    class_select.with_db(&*db).ui(ui);
                })
        });
    }
}

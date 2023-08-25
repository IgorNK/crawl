// Courtesy of gzp79 @ https://github.com/gzp79/shine/tree/main

use eframe::egui;
use std::sync::Arc;
use crate::View;

#[derive(Clone)]
pub struct ZoomPanState {
  pub id: egui::Id,
  pub pan: egui::Vec2,
  pub zoom: f32,
  pub screen_rect: egui::Rect,
  pub default_style: Arc<egui::Style>,
  pub zoomed_style: Arc<egui::Style>,
}

impl ZoomPanState {
  pub fn new(id: egui::Id) -> Self {
    ZoomPanState {
      id,
      pan: egui::Vec2::ZERO,
      zoom: 1f32,
      screen_rect: egui::Rect::NOTHING,
      default_style: Default::default(),
      zoomed_style: Default::default(),
    }
  }

  pub fn pos2_area_to_screen(&self, p: egui::Pos2) -> egui::Pos2 {
    let egui::Pos2 { x, y } = p;
    let x = x + self.screen_rect.left();
    let y = y + self.screen_rect.top();
    let x = (x + self.pan.x) * self.zoom;
    let y = (y + self.pan.y) * self.zoom;
    egui::pos2(x, y)
  }

  pub fn vec2_screen_to_area(&self, v: egui::Vec2) -> egui::Vec2 {
    egui::vec2(v.x / self.zoom, v.y / self.zoom)
  }

  pub fn pos2_screen_to_area(&self, p: egui::Pos2) -> egui::Pos2 {
    let egui::Pos2 { x, y } = p;
    let x = x / self.zoom - self.pan.x;
    let y = y / self.zoom - self.pan.y;
    let x = x - self.screen_rect.left();
    let y = y - self.screen_rect.top();
    egui::pos2(x, y)
  }

  pub fn drag(&mut self, delta: egui::Vec2) {
    let delta = self.vec2_screen_to_area(delta);
    self.update(self.pan + delta, self.zoom);
  }

  pub fn zoom_to_screen(&mut self, screen_pos: egui::Pos2, zoom: f32) {
    let new_zoom = (self.zoom * zoom).clamp(0.1f32, 10f32);

    let test = self.pos2_screen_to_area(screen_pos);

    let egui::Pos2 { x, y } = screen_pos;
    let new_pan = egui::vec2(
      x / new_zoom - x / self.zoom + self.pan.x,
      y / new_zoom - y / self.zoom + self.pan.y,
    );

    let err = self.pos2_area_to_screen(test) - screen_pos;
    assert!(err.x < 1f32);
    assert!(err.y < 1f32);
    self.update(new_pan, new_zoom);
  }

  pub fn prepare(&mut self, style: &Arc<egui::Style>) {
    self.default_style = style.clone();
  }

  pub fn update(&mut self, pan: egui::Vec2, zoom: f32) {
    //if self.zoom != zoom {
    //  self.zoomed_style = Arc::new(self.default_style.scaled(self.zoom));
    //}
    self.pan = pan;
    self.zoom = zoom;
  }

  pub fn show_zoomed<R, F>(&self, ui: &mut egui::Ui, add_content: F) -> R
  where
    F: FnOnce(&mut egui::Ui) -> R,
  {
    let original_cliprect = ui.clip_rect();
    ui.set_clip_rect(self.screen_rect);
    ui.ctx().set_style(self.zoomed_style.clone());
    let response = add_content(ui);
    ui.ctx().set_style(self.default_style.clone());
    ui.set_clip_rect(original_cliprect);

    response
  }

  pub fn show_clipped<R, F>(&self, ui: &mut egui::Ui, add_content: F) -> R
  where
    F: FnOnce(&mut egui::Ui) -> R,
  {
    let original_cliprect = ui.clip_rect();
    ui.set_clip_rect(self.screen_rect);
    let response = add_content(ui);
    ui.set_clip_rect(original_cliprect);

    response
  }
}

pub trait ZoomPan: View {
  fn zoom_pan_state(&mut self) -> &mut ZoomPanState;
  fn id(&self) -> &egui::Id;
  
  fn ui_with_zoom_pan(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
    self.zoom_pan_state().prepare(ui.style());
    self.zoom_pan_state().screen_rect = ui.available_rect_before_wrap();

    let mut response = ui.interact(self.zoom_pan_state().screen_rect, self.id().clone(), egui::Sense::drag());

    // handle pan
    self.zoom_pan_state().drag(response.drag_delta());

    // handle zoom
    if let Some(pos) = ui.ctx().pointer_latest_pos() {
      let zoom = ui.input(|i| i.scroll_delta.y);
      if zoom != 0f32 && self.zoom_pan_state().screen_rect.contains(pos) {
        log::warn!("zooming to {}", zoom);
        let zoom = (zoom * 0.002f32).exp();
        self.zoom_pan_state().zoom_to_screen(pos, zoom);
      }
    }

    self.zoom_pan_state().show_zoomed(ui, |ui| {
      self.ui(ui, ctx);
    });
  
  }
}
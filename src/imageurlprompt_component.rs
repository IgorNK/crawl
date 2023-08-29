use crate::api;
use bytes::Bytes;
use eframe::egui;
use std::sync::{mpsc, Arc};

#[derive(Debug)]
pub struct ImageUrlPromptComponent {
    url: String,
    sender: Option<mpsc::Sender<Arc<Bytes>>>,
}

impl Default for ImageUrlPromptComponent {
    fn default() -> Self {
        ImageUrlPromptComponent {
            url: String::new(),
            sender: None,
        }
    }
}

impl ImageUrlPromptComponent {
    pub fn with_sender(self, sender: mpsc::Sender<Arc<Bytes>>) -> Self {
        Self {
            sender: Some(sender),
            ..self
        }
    }

    pub fn set_sender(&mut self, sender: mpsc::Sender<Arc<Bytes>>) {
        self.sender = Some(sender);
    }
}

impl crate::Window for ImageUrlPromptComponent {
    fn name(&self) -> &'static str {
        "Image URL"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        use crate::View as _;
        egui::Window::new(self.name())
            .open(open)
            .default_height(500.0)
            .show(ctx, |ui| self.ui(ui, ctx));
    }
}

impl crate::View for ImageUrlPromptComponent {
    fn ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.label("URL: ");
        if ui.text_edit_singleline(&mut self.url).lost_focus()
            && ui.input(|i| i.key_pressed(egui::Key::Enter))
        {
            dbg!(&self);
            dbg!(&self.sender);
            if let Some(ref mut sender) = self.sender {
                dbg!(&self.url);
                #[cfg(not(target_arch = "wasm32"))]
                api::fetch_image(self.url.clone(), sender.clone());

                #[cfg(target_arch = "wasm32")]
                api::fetch_image_web(self.url.clone(), sender.clone());
            }
        }
    }
}

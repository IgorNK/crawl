use eframe::egui;
use std::sync::{mpsc, Arc};
use crate::api::{fetch_image, fetch_image_web};

pub struct ImageUrlPromptComponent {
    url: String,
    sender: Option<mpsc::Sender<Arc<[u8]>>>,
}

impl Default for ImageUrlPromptComponent {
    fn default() -> Self {
        ImageUrlPromptComponent {
            url: String::new(),
            sender: None,
        }
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
        let Self {
            ref mut url,
            sender,
        } = self;
      
        ui.label("URL: ");
        if ui.text_edit_singleline(url).lost_focus()
            && ui.input(|i| i.key_pressed(egui::Key::Enter))
        {
          let mut image_bytes;
          
          #[cfg(not(target_arch = "wasm32"))]
          image_bytes = fetch_image(url);
          
          #[cfg(target_arch = "wasm32")]
          image_bytes = fetch_image_web(url);

          if let Ok(image_bytes) = image_bytes {
            sender.send(image_bytes);
          }

          sender = None;
        }
    }
}

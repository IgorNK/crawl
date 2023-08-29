use crate::View;
use crate::Window;
use eframe::egui;
use std::collections::BTreeSet;

pub struct Windows {
    windows: Vec<Box<dyn Window>>,
    pub open: BTreeSet<String>,
}

impl Default for Windows {
    fn default() -> Self {
        dbg!("call to default window creation");
        Windows::from_windows(vec![
            Box::<crate::character_gen::CharacterGenComponent>::default(),
            Box::<crate::chat_component::ChatComponent>::default(),
            Box::<crate::imageurlprompt_component::ImageUrlPromptComponent>::default(),
        ])
    }
}

impl Windows {
    pub fn from_windows(windows: Vec<Box<dyn Window>>) -> Self {
        let open = BTreeSet::new();
        Self { windows, open }
    }

    pub fn windows(&mut self, ctx: &egui::Context) {
        let Self { windows, open } = self;
        for window in windows {
            let mut is_open = open.contains(window.name());
            window.show(ctx, &mut is_open);
            set_open(open, window.name(), is_open);
        }
    }

    pub fn get_mut(&mut self, key: &'static str) -> Option<&mut Box<dyn Window>> {
      for window in &mut windows {
        if window.name() == key {
          Some(window)
        }
      }
      None
    }
}
pub fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}

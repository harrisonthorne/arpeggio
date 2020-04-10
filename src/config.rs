use std::collections::HashMap;
use crate::palette::Palette;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    palettes: HashMap<String, Palette>,
}

impl Config {
    pub fn set_palette(&mut self, path: String, palette: Palette) {
        self.palettes.insert(path, palette);
    }
}
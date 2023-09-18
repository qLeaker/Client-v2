use std::collections::HashMap;
use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct QleakerApp {
    pub(crate) list: Vec<GarrysModAddon>,
    pub(crate) search: String,
    pub(crate) texture: Vec<Option<egui::TextureHandle>>,
}

pub enum Msg {
    Refresh,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct GarrysModAddonList {
    pub(crate) lists: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct GarrysModAddon {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) image: String,
    pub(crate) file: String,
    pub(crate) store: String,
    pub(crate) content: String,
}

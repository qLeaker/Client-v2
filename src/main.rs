#![windows_subsystem = "windows"]

mod garrys_mod_database;
mod req;
mod structurs;
mod hex_utils;


use std::clone::Clone;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write};
use std::sync::Arc;
use std::thread;
use std::thread::{JoinHandle};
use crate::structurs::{GarrysModAddon, QleakerApp};
use eframe::{egui};
use eframe::egui::{Context, Rect, Ui};
use once_cell::sync::Lazy;
use serde_json::Value;

use crate::garrys_mod_database::GarrysModDatabase;
use crate::hex_utils::{load_image_from_memory, string_to_hex};
use crate::req::{request_get, request_get_image};

const GMOD_PREFIX: &str = "[Garry's mod]";

fn load_icon() -> eframe::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image_dst = request_get_image(String::from("https://github.com/qLeaker/Client-v2/releases/download/icon/package.png")).unwrap();
        let image = image::load_from_memory(image_dst.as_slice()).unwrap().into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1052.0, 505.0)),
        // resizable: false,
        icon_data: Some(load_icon()),
        ..Default::default()
    };
    let result = eframe::run_native(
        "qLeaker",
        options,
        Box::new(|cc|  unsafe {
            let clone = cc.egui_ctx.clone();
            EMGUI_CONTEXT = core::option::Option::from(clone);
            Box::<QleakerApp>::default() }
        ),
    );
    result
}
static mut EMGUI_CONTEXT: Option<Context> = None;

static mut ALL_TEXTURES: Lazy<HashMap<String, egui::TextureHandle>> = Lazy::new(|| {
    let map = HashMap::<String, egui::TextureHandle>::new();
    map
});

static mut OBJ: QleakerApp = QleakerApp {
    list: vec![],
    search: String::new(),
    texture: vec![],
};

static mut LOADED_ADDONS: Lazy<Arc<bool>> = Lazy::new(|| {
    Arc::new(false)
});


impl Default for QleakerApp {
    fn default() -> Self {
        let mut database1 = Self {
            list: vec![],
            search: "".to_string(),
            texture: vec![],
        };
        for _i in 0..10000 {
            database1.texture.push(None);
        }
        let database3 = database1.clone();

        let _thread = thread::spawn(move || unsafe {
            let mut i = 0;

            let mut threads: std::vec::Vec<JoinHandle<bool>>= vec![];

            let database = GarrysModDatabase::new();
            for addon in database.rx {
                let addon_copy = addon.clone();
                let addon_copy2 = addon.clone();
                OBJ.list.push(addon);
                let mut database4 = database3.clone();
                let _thread = thread::spawn(move || {

                    i = i + 1;
                    let image_dst = request_get_image(addon_copy.image);
                    match image_dst {
                        None => { return false; }
                        Some(dist) => {
                            let name = addon_copy2.name.clone();
                            let image = load_image_from_memory(dist.as_slice());
                            match image {
                                Ok(img) => {
                                    match EMGUI_CONTEXT.as_mut() {
                                        None => {}
                                        Some(a) => {
                                            let texture: &egui::TextureHandle = database4.texture[i].get_or_insert_with(|| {
                                                a.load_texture(addon_copy.name, img, Default::default())
                                            });
                                            let bb = texture.clone();
                                            let nn = name.trim().to_string();
    
                                            ALL_TEXTURES.insert(nn, bb);
                                            return true;
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("Ошибка картинки {}", e);
                                    return false;
                                }
                            }
                        }
                    }
                    return false;
                });
                threads.push(_thread);
            }

            for threadh in threads {
                let _j = threadh.join();
            }
            *LOADED_ADDONS = true.into();
        });

        return database1
    }
}
unsafe fn add_gmod(addon: &GarrysModAddon, ui: &mut Ui, _rect: Rect) {
    let texture_handle = ALL_TEXTURES.get(addon.name.trim());
    match texture_handle {
        None => { }
        Some(texture) => {
            ui.image(texture, (250.0, 75.0));
            ui.set_width(250.0);
            ui.label(addon.name.as_str());
            ui.set_width(250.0);
            ui.label(format!("Version: {}", addon.version.as_str()));
            ui.horizontal(|ui| {
                if ui.button("Download").clicked() {

                    match request_get(&*urlencoding::decode(addon.file.as_str()).unwrap()) {
                        None => { return; }
                        Some(body) => {
                            match serde_json::from_str::<Value>(body.as_str()) {
                                Ok(p) => {
                                    let hexstr = p["file"].as_str();


                                    match hexstr {
                                        None => {
                                            println!("File not found")
                                        }
                                        Some(d) => {
                                            let dst = string_to_hex(d.to_string());

                                            match File::create(format!("{}.zip", addon.name)) {
                                                Ok(mut a) => {
                                                    match a.write_all(dst.as_slice()) {
                                                        Ok(_) => {
                                                            println!("File downloaded")
                                                        }
                                                        Err(_) => {
                                                            println!("File write error")
                                                        }
                                                    }
                                                }
                                                Err(_) => {
                                                    println!("error #2")
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("Ошибка конвертаций в Json: {}", e)
                                }
                            }
                        }
                    }
                }
                if !addon.content.is_empty() {
                    if ui.button("Content").clicked() {
                        let trimed = addon.content.trim();
                        if trimed.contains(" ") {
                            let splited = trimed.split(" ");
                            for content_url in splited {
                                match open::that(content_url) {
                                    Ok(()) => {},
                                    Err(_err) => {},
                                }
                            }
                        }
                        else {
                            match open::that(trimed) {
                                Ok(()) => {},
                                Err(_err) => {},
                            }
                        }
                    }
                }
                if !addon.store.is_empty() {
                    if ui.button("GmodStore").clicked() {
                        match open::that(addon.store.as_str()) {
                            Ok(()) => {},
                            Err(_err) => {},
                        }
                    }
                }
            });
        }
    }
}

impl eframe::App for QleakerApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| unsafe {


            ui.horizontal(|ui| {
                // ui.set_width(225.00);
                ui.label("Search:");
                ui.text_edit_singleline(&mut OBJ.search);
            });
            ui.separator();
            let rect = ui.min_rect();
            if **LOADED_ADDONS {
                egui::ScrollArea::new([true, true]).show(ui, |ui| {
                    let mut list = OBJ.list.clone();
                    let filtered_list: Vec<GarrysModAddon> = list
                        .drain(..)
                        .filter(|addon| addon.name.to_lowercase().contains(OBJ.search.to_lowercase().as_str()))
                        .collect();
                    let chinks = filtered_list.chunks((rect.width() / 259.0) as usize);
                    for addon in chinks {
                        ui.horizontal(|ui| {
                            for addonun in addon {
                                ui.vertical(|ui| {
                                    add_gmod(addonun, ui, rect);
                                    ui.separator();
                                });
                            }
                        });
                    }
                });
            }
        });
    }
}
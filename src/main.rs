mod garrys_mod_database;
mod req;
mod structurs;
mod hex_utils;


use std::clone::Clone;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use easy_hex::{HexExt};
use crate::structurs::{GarrysModAddon, QleakerApp};
use eframe::{egui};
use eframe::egui::{Context, TextBuffer};
use image::{ImageDecoder};
use once_cell::sync::Lazy;
use serde_json::Value;

use crate::garrys_mod_database::GarrysModDatabase;
use crate::hex_utils::{load_image_from_memory, string_to_hex};
use crate::req::{into_string, request_get, request_get_image};

const GMOD_PREFIX: &str = "[Garry's mod]";
const REQUEST_PREFIX: &str = "[Garry's mod]";

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(278.0, 460.0)),
        resizable: false,
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

static mut ALL_BOOKS_BY_SHORT_NAME: Lazy<HashMap<String, egui::TextureHandle>> = Lazy::new(|| {
    let mut map = HashMap::<String, egui::TextureHandle>::new();
    map
});

static mut OBJ: QleakerApp = QleakerApp {
    list: vec![],
    search: String::new(),
    texture: vec![],
};

impl Default for QleakerApp {
    fn default() -> Self {
        let mut database1 = Self {
            list: vec![],
            search: "".to_string(),
            texture: vec![],
        };
        for i in 0..10000 {
            database1.texture.push(None);
        }
        let mut database3 = database1.clone();

        let _thread = thread::spawn(move || unsafe {

            // let mut database2 = rx1.recv().unwrap();


            let mut i = 0;
            let database = GarrysModDatabase::new();
            for addon in database.rx {
                let addon_copy = addon.clone();
                let addon_copy2 = addon.clone();

                let mut database4 = database3.clone();
                OBJ.list.push(addon);

                let _thread = thread::spawn(move || unsafe {

                    i = i + 1;
                    sleep(Duration::from_millis(25));
                    let mut image_dst = request_get_image(addon_copy.image);
                    match image_dst {
                        None => {}
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
                                            ALL_BOOKS_BY_SHORT_NAME.insert(nn, bb);
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("Ошибка картинки {}", e)
                                }
                            }
                        }
                    }
                });
            }
        });

        return database1
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

            let mut row = 4;
            let mut cel = 0;

            egui::ScrollArea::new([true, true]).show(ui, |ui| {
                ui.vertical(|ui| {
                    let mut list = OBJ.list.clone();
                    let the_vocabulary: Vec<GarrysModAddon> = list
                        .drain(..)
                        .filter(|voc| voc.name.to_lowercase().contains(OBJ.search.to_lowercase().as_str()))
                        .collect();
                    for addon in the_vocabulary {
                        ui.vertical(|ui| {
                            let a = ALL_BOOKS_BY_SHORT_NAME.get(addon.name.trim());
                            match a {
                                None => { }
                                Some(a) => {
                                    ui.image(a, (250.0, 75.));
                                    ui.set_width(250.0);
                                    ui.label(addon.name.as_str());
                                }
                            }
                        });
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                if ui.button("Download").clicked() {
                                    println!("Скачиваю: {}", urlencoding::decode(addon.file.as_str()).unwrap());
                                    match request_get(&*urlencoding::decode(addon.file.as_str()).unwrap()) {
                                        None => { todo!() }
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
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                                if ui.button("Content").clicked() {
                                    match open::that(addon.content.as_str()) {
                                        Ok(()) => {},
                                        Err(_err) => {},
                                    }
                                }
                                if ui.button("GmodStore").clicked() {
                                    match open::that(addon.store.as_str()) {
                                        Ok(()) => {},
                                        Err(_err) => {},
                                    }
                                }
                            });
                        });
                        ui.separator();
                    }
                });
            });
        });
    }
}
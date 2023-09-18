use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver};
use crate::structurs::{GarrysModAddon, GarrysModAddonList};
use std::thread;
use std::thread::{JoinHandle};

use crate::GMOD_PREFIX;

use crate::req::request_get;

const GMOD_LIST_URL: &str = "https://github.com/qLeaker/Cloud/raw/main/gmod/list.json";

pub struct GarrysModDatabase {
    pub(crate) addons: Vec<GarrysModAddon>,
    pub(crate) rx: Receiver<GarrysModAddon>
}
impl GarrysModDatabase{
    pub(crate) fn new() -> GarrysModDatabase {
        let (tx, rx1) = channel();
        let database = Self {
            addons: vec![],
            rx: rx1
        };
        let data = Arc::new(Mutex::new(0));
        let (_data, tx) = (Arc::clone(&data), tx.clone());
        let mut threads: Vec<JoinHandle<GarrysModAddon>> = vec![];

        let Some(list_string) = request_get(GMOD_LIST_URL) else {
            return database
        };
        let Ok(list) = serde_json::from_str::<GarrysModAddonList>(list_string.as_str()) else {
            return database
        };
        for addon_url in list.lists {
            let thread = thread::spawn(move || {
                let Some(addon_string) = request_get(addon_url.as_str(), ) else {
                    return GarrysModAddon {
                        name: "".to_string(),
                        version: "".to_string(),
                        image: "".to_string(),
                        file: "".to_string(),
                        store: "".to_string(),
                        content: "".to_string(),
                    };
                };
                let Ok(addon) = serde_json::from_str::<GarrysModAddon>(addon_string.as_str()) else {
                    println!("{0} Ошибка загрузки: {1}", GMOD_PREFIX, addon_url);
                    return GarrysModAddon {
                        name: "".to_string(),
                        version: "".to_string(),
                        image: "".to_string(),
                        file: "".to_string(),
                        store: "".to_string(),
                        content: "".to_string(),
                    };
                };
                // println!("{0} Загружаю аддон: {1}", GMOD_PREFIX, addon.name);
                addon

            });
            threads.push(thread);
        }

        for thead in threads {
            let addon = thead.join().unwrap();
            tx.send(addon).unwrap();
        }
        database
    }
}

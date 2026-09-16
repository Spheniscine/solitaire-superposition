use dioxus::logger::tracing;
use web_sys::{window, Storage};

use crate::game::GameState;

pub struct LocalStorage;

const KEY: &str = "solitaire-superposition-state";

impl LocalStorage {
    fn get_storage(&self) -> Result<Option<Storage>, String> {
        let window = window().ok_or("can't get window".to_string())?;
        let storage = window.local_storage().map_err(|_| "can't get storage".to_string())?;
        Ok(storage)
    }

    pub fn save_game_state(&self, state: &GameState) {
        match self.get_storage() {
            Ok(Some(storage)) => {
                match serde_json5::to_string(state) {
                    Ok(result) => {storage.set_item(KEY, &result).ok();}
                    Err(e) => {tracing::error!("{:?}", e);}
                }
            }
            Ok(None) => {
                tracing::error!("storage not enabled");
            }
            Err(e) => {
                tracing::error!("{}", e);
            }
        }
    }

    pub fn load_game_state(&self) -> Option<GameState> {
        match self.get_storage() {
            Ok(Some(storage)) => {
                match storage.get_item(KEY) {
                    Ok(Some(string)) => {
                        if let Ok(res) = serde_json5::from_str::<GameState>(&string) {
                            return Some(res);
                        }
                    }
                    _ => {}
                }
            }
            Ok(None) => {
                tracing::error!("storage not enabled");
            }
            Err(e) => {
                tracing::error!("{}", e);
            }
        }
        None
    }
}
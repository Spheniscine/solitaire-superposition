use serde::{Deserialize, Serialize};

use crate::game::Skin;

#[derive(Clone, Serialize, Deserialize)]
pub struct SettingsState {
    pub allow_undo: bool,
    pub skin: Skin,
}
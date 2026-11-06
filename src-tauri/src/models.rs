use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WallpaperEntry {
    pub name: String,
    pub path: String,
}

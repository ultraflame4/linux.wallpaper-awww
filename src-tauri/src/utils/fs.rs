use std::path::PathBuf;

/// Returns the path to the wallpapers directory: `$HOME/Pictures/Wallpapers`.
pub fn wallpapers_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home".into());
    PathBuf::from(home).join("Pictures").join("Wallpapers")
}

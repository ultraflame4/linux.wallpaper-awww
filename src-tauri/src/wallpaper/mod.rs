use std::path::PathBuf;

pub mod awww;

/// Represents a wallpaper backend that can set and query the current wallpaper.
pub trait WallpaperBackend: Send + Sync {
    /// Set the wallpaper to the given image path.
    fn set(&self, path: &PathBuf) -> Result<(), String>;

    /// Get the path of the currently active wallpaper, if any.
    fn current(&self) -> Result<Option<PathBuf>, String>;

    /// Human-readable name for the backend (e.g. "awww").
    fn name(&self) -> &'static str;
}

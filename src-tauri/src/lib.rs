mod commands;
mod models;
mod utils;
mod wallpaper;

use std::path::PathBuf;
use std::sync::Mutex;

use wallpaper::WallpaperBackend;
use wallpaper::awww::AwwwBackend;

pub struct AppState {
    backend: Box<dyn WallpaperBackend>,
    current_wallpaper: Mutex<Option<PathBuf>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let backend: Box<dyn WallpaperBackend> = if AwwwBackend::is_available() {
        let b = AwwwBackend::new();
        println!("Wallpaper backend: {}", b.name());
        Box::new(b)
    } else {
        eprintln!("Warning: awww not found. Wallpaper setting will not work.");
        struct NoopBackend;
        impl WallpaperBackend for NoopBackend {
            fn set(&self, _path: &PathBuf) -> Result<(), String> {
                Err("No wallpaper backend available. Please install awww.".into())
            }
            fn current(&self) -> Result<Option<PathBuf>, String> {
                Ok(None)
            }
            fn name(&self) -> &'static str {
                "none"
            }
        }
        Box::new(NoopBackend)
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            backend,
            current_wallpaper: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_wallpapers,
            commands::set_wallpaper,
            commands::get_current_wallpaper,
            commands::download_wallpaper,
            commands::read_image,
            commands::open_wallpapers_folder,
            commands::open_in_folder,
            commands::preview_url,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

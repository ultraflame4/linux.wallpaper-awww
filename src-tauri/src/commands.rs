use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::models::WallpaperEntry;
use crate::utils::download;
use crate::utils::fs::wallpapers_dir;
use crate::utils::image;

use crate::AppState;

#[tauri::command]
pub fn list_wallpapers() -> Result<Vec<WallpaperEntry>, String> {
    let dir = wallpapers_dir();
    let mut entries = Vec::new();

    let dir_entries = fs::read_dir(&dir).map_err(|e| format!("Cannot read wallpapers dir: {e}"))?;

    for entry in dir_entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {e}"))?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let ext_lower = ext.to_lowercase();
                if matches!(
                    ext_lower.as_str(),
                    "jpg" | "jpeg" | "png" | "webp" | "bmp" | "gif"
                ) {
                    entries.push(WallpaperEntry {
                        name: path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned(),
                        path: path.to_string_lossy().into_owned(),
                    });
                }
            }
        }
    }

    entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(entries)
}

#[tauri::command]
pub fn set_wallpaper(path: String, state: tauri::State<'_, AppState>) -> Result<String, String> {
    let path = PathBuf::from(&path);

    if !path.exists() {
        return Err(format!("File not found: {}", path.display()));
    }

    state.backend.set(&path)?;

    if let Ok(mut current) = state.current_wallpaper.lock() {
        *current = Some(path.clone());
    }

    Ok(format!("Wallpaper set to {}", path.display()))
}

#[tauri::command]
pub fn get_current_wallpaper(state: tauri::State<'_, AppState>) -> Result<Option<String>, String> {
    if let Ok(Some(path)) = state.backend.current() {
        return Ok(Some(path.to_string_lossy().into_owned()));
    }
    if let Ok(guard) = state.current_wallpaper.lock() {
        return Ok(guard.as_ref().map(|p| p.to_string_lossy().into_owned()));
    }
    Ok(None)
}

#[tauri::command]
pub async fn download_wallpaper(url: String) -> Result<WallpaperEntry, String> {
    let dir = wallpapers_dir();
    fs::create_dir_all(&dir).map_err(|e| format!("Cannot create wallpapers dir: {e}"))?;

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Download failed: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()));
    }

    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read response: {e}"))?;

    download::validate_image(&bytes)?;

    let filename = download::derive_filename(&url, &content_type, &bytes);
    let filepath = dir.join(&filename);

    fs::write(&filepath, &bytes).map_err(|e| format!("Failed to save file: {e}"))?;

    Ok(WallpaperEntry {
        name: filename,
        path: filepath.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
pub fn read_image(path: String) -> Result<String, String> {
    let path = PathBuf::from(&path);
    if !path.exists() {
        return Err(format!("File not found: {}", path.display()));
    }

    let bytes = fs::read(&path).map_err(|e| format!("Cannot read file: {e}"))?;

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("jpg")
        .to_lowercase();

    let mime = image::ext_to_mime(&ext);
    Ok(image::encode_data_url(&bytes, mime))
}

#[tauri::command]
pub fn open_wallpapers_folder() -> Result<(), String> {
    let dir = wallpapers_dir();
    fs::create_dir_all(&dir).map_err(|e| format!("Cannot create dir: {e}"))?;
    Command::new("xdg-open")
        .arg(&dir)
        .spawn()
        .map_err(|e| format!("Failed to open folder: {e}"))?;
    Ok(())
}

#[tauri::command]
pub fn open_in_folder(path: String) -> Result<(), String> {
    let path = PathBuf::from(&path);
    let parent = path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("/"));
    Command::new("xdg-open")
        .arg(&parent)
        .spawn()
        .map_err(|e| format!("Failed to open folder: {e}"))?;
    Ok(())
}

#[tauri::command]
pub async fn preview_url(url: String) -> Result<String, String> {
    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to fetch URL: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()));
    }

    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("image/jpeg")
        .to_string();

    if !content_type.starts_with("image/") {
        return Err("URL does not point to an image".into());
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read response: {e}"))?;

    download::validate_image(&bytes)?;

    Ok(image::encode_data_url(&bytes, &content_type))
}

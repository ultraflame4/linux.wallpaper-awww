mod wallpaper;

use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

use base64::Engine;

use wallpaper::WallpaperBackend;
use wallpaper::awww::AwwwBackend;

struct AppState {
    backend: Box<dyn WallpaperBackend>,
    current_wallpaper: Mutex<Option<PathBuf>>,
}

fn wallpapers_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home".into());
    PathBuf::from(home).join("Pictures").join("Wallpapers")
}

#[tauri::command]
fn list_wallpapers() -> Result<Vec<WallpaperEntry>, String> {
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
    println!("Found wallpapers: {entries:#?}");
    Ok(entries)
}

#[tauri::command]
fn set_wallpaper(path: String, state: tauri::State<'_, AppState>) -> Result<String, String> {
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
fn get_current_wallpaper(state: tauri::State<'_, AppState>) -> Result<Option<String>, String> {
    // First try to get it from the backend (e.g. swww query)
    if let Ok(Some(path)) = state.backend.current() {
        return Ok(Some(path.to_string_lossy().into_owned()));
    }
    // Fall back to in-memory state
    if let Ok(guard) = state.current_wallpaper.lock() {
        return Ok(guard.as_ref().map(|p| p.to_string_lossy().into_owned()));
    }
    Ok(None)
}

#[tauri::command]
async fn download_wallpaper(url: String) -> Result<WallpaperEntry, String> {
    let dir = wallpapers_dir();
    fs::create_dir_all(&dir).map_err(|e| format!("Cannot create wallpapers dir: {e}"))?;

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Download failed: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()));
    }

    // Derive filename from URL or Content-Disposition header
    let filename = derive_filename(&url, &response);
    let filepath = dir.join(&filename);

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read response: {e}"))?;

    // Basic validation: check magic bytes for common image formats
    validate_image(&bytes)?;

    fs::write(&filepath, &bytes).map_err(|e| format!("Failed to save file: {e}"))?;

    Ok(WallpaperEntry {
        name: filename,
        path: filepath.to_string_lossy().into_owned(),
    })
}

fn derive_filename(url: &str, response: &reqwest::Response) -> String {
    // Try Content-Disposition header first
    if let Some(disp) = response.headers().get("content-disposition") {
        if let Ok(disp_str) = disp.to_str() {
            for part in disp_str.split(';') {
                let part = part.trim();
                if let Some(fname) = part.strip_prefix("filename=") {
                    let fname = fname.trim_matches('"').trim_matches('\'');
                    if !fname.is_empty() {
                        return sanitize_filename(fname);
                    }
                }
            }
        }
    }

    // Fall back to URL path segment
    let path = url.split('?').next().unwrap_or(url);
    if let Some(segment) = path.rsplit('/').next() {
        if !segment.is_empty() {
            let decoded = urlencoding::decode(segment).unwrap_or_else(|_| segment.into());
            return sanitize_filename(&decoded);
        }
    }

    // Final fallback
    format!("wallpaper_{}.jpg", chrono::Utc::now().timestamp())
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

fn validate_image(bytes: &[u8]) -> Result<(), String> {
    if bytes.is_empty() {
        return Err("Empty response body".into());
    }
    // Check common image magic bytes
    let is_image = bytes.starts_with(b"\xFF\xD8\xFF")      // JPEG
        || bytes.starts_with(b"\x89PNG\r\n\x1A\n")       // PNG
        || bytes.starts_with(b"RIFF") && bytes.len() > 8  // WEBP
        || bytes.starts_with(b"BM")                         // BMP
        || bytes.starts_with(b"GIF8"); // GIF

    if is_image {
        Ok(())
    } else {
        Err("Downloaded file does not appear to be an image".into())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
struct WallpaperEntry {
    name: String,
    path: String,
}

#[tauri::command]
fn read_image(path: String) -> Result<String, String> {
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

    let mime = match ext.as_str() {
        "png" => "image/png",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        "gif" => "image/gif",
        _ => "image/jpeg",
    };

    let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
    Ok(format!("data:{mime};base64,{b64}"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let backend: Box<dyn WallpaperBackend> = if AwwwBackend::is_available() {
        let b = AwwwBackend::new();
        println!("Wallpaper backend: {}", b.name());
        Box::new(b)
    } else {
        eprintln!("Warning: awww not found. Wallpaper setting will not work.");
        // Use a no-op backend that just logs errors
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
            list_wallpapers,
            set_wallpaper,
            get_current_wallpaper,
            download_wallpaper,
            read_image,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

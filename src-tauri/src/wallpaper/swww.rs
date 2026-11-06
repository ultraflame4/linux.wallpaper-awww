use std::path::PathBuf;
use std::process::Command;

use crate::wallpaper::WallpaperBackend;

pub struct SwwwBackend;

impl SwwwBackend {
    pub fn new() -> Self {
        Self
    }

    /// Check whether `swww` is available on the system.
    pub fn is_available() -> bool {
        Command::new("which")
            .arg("swww")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

impl WallpaperBackend for SwwwBackend {
    fn set(&self, path: &PathBuf) -> Result<(), String> {
        let output = Command::new("swww")
            .arg("img")
            .arg(path)
            .arg("--transition-type")
            .arg("center")
            .output()
            .map_err(|e| format!("Failed to run swww: {e}"))?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("swww failed: {stderr}"))
        }
    }

    fn current(&self) -> Result<Option<PathBuf>, String> {
        let output = Command::new("swww")
            .arg("query")
            .output()
            .map_err(|e| format!("Failed to run swww query: {e}"))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        // swww query outputs lines like: "DP-1: /path/to/image"
        for line in stdout.lines() {
            if let Some((_, path_str)) = line.split_once(':') {
                let path_str = path_str.trim();
                if !path_str.is_empty() {
                    return Ok(Some(PathBuf::from(path_str)));
                }
            }
        }
        Ok(None)
    }

    fn name(&self) -> &'static str {
        "swww"
    }
}

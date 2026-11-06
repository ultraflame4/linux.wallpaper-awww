use std::path::PathBuf;
use std::process::Command;

use crate::wallpaper::WallpaperBackend;

pub struct AwwwBackend;

impl AwwwBackend {
    pub fn new() -> Self {
        Self
    }

    /// Check whether `awww` is available on the system.
    pub fn is_available() -> bool {
        Command::new("which")
            .arg("awww")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

impl WallpaperBackend for AwwwBackend {
    fn set(&self, path: &PathBuf) -> Result<(), String> {
        let output = Command::new("awww")
            .arg("img")
            .arg(path)
            .arg("--transition-type")
            .arg("center")
            .output()
            .map_err(|e| format!("Failed to run awww: {e}"))?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("awww failed: {stderr}"))
        }
    }

    fn current(&self) -> Result<Option<PathBuf>, String> {
        let output = Command::new("awww")
            .arg("query")
            .output()
            .map_err(|e| format!("Failed to run awww query: {e}"))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        // awww query outputs lines like: "DP-1: /path/to/image"
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
        "awww"
    }
}

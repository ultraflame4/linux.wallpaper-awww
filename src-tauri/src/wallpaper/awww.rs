use std::path::PathBuf;
use std::process::Command;

use jsonpath_rust::JsonPath;
use serde_json::Value;

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
            .arg("--json")
            .output()
            .map_err(|e| format!("Failed to run awww query: {e}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("awww query failed: {stderr}"));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let json: Value =
            serde_json::from_str(&stdout).map_err(|e| format!("Failed to parse awww JSON: {e}"))?;

        let find = JsonPath::try_from("$.*[*].displaying.image")
            .map_err(|e| format!("Invalid jsonpath: {e}"))?
            .find(&json);
        let Some(image) = find.as_array() else {
            return Err(format!("Expected array"));
        };

        if image.len() == 0 {
            return Ok(None);
        }

        let value = image[0].as_str();
        Ok(value.map(PathBuf::from))
    }

    fn name(&self) -> &'static str {
        "awww"
    }
}

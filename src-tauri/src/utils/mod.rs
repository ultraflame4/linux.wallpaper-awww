pub mod download;
pub mod fs;
pub mod image;

/// Known/supported image file extensions.
pub const KNOWN_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "webp", "bmp", "gif"];

/// Returns true if the given extension is a known image format.
pub fn is_known_ext(ext: &str) -> bool {
    KNOWN_EXTENSIONS
        .iter()
        .any(|&e| e.eq_ignore_ascii_case(ext))
}

/// Maps a Content-Type header value to a file extension.
pub fn mime_to_ext(content_type: &str) -> &str {
    if content_type.contains("image/png") {
        "png"
    } else if content_type.contains("image/webp") {
        "webp"
    } else if content_type.contains("image/bmp") {
        "bmp"
    } else if content_type.contains("image/gif") {
        "gif"
    } else if content_type.contains("image/jpeg") || content_type.contains("image/jpg") {
        "jpg"
    } else {
        "jpg"
    }
}

/// Detects the image file extension from magic bytes.
pub fn detect_ext_from_bytes(bytes: &[u8]) -> &str {
    if bytes.starts_with(b"\x89PNG") {
        "png"
    } else if bytes.starts_with(b"RIFF") && bytes.len() > 8 {
        "webp"
    } else if bytes.starts_with(b"BM") {
        "bmp"
    } else if bytes.starts_with(b"GIF8") {
        "gif"
    } else {
        "jpg"
    }
}

/// Extracts the file extension from a filename (the part after the last dot).
fn parse_ext_from_filename(name: &str) -> Option<&str> {
    name.rfind('.').map(|pos| &name[pos + 1..])
}

/// Normalizes jpeg/jpg to "jpg" for comparison purposes.
fn normalize_ext(ext: &str) -> &str {
    if ext.eq_ignore_ascii_case("jpeg") {
        "jpg"
    } else {
        ext
    }
}

/// Derives a safe filename from a URL, using the Content-Type and magic bytes
/// to determine the correct extension. Only appends the detected extension if
/// the filename has no extension, an unrecognised extension, or an extension
/// that differs from the detected format (treating jpeg/jpg as equivalent).
pub fn derive_filename(url: &str, content_type: &str, bytes: &[u8]) -> String {
    let detected_ext = if !content_type.is_empty() {
        mime_to_ext(content_type)
    } else {
        detect_ext_from_bytes(bytes)
    };

    let base = url
        .split('?')
        .next()
        .unwrap_or(url)
        .rsplit('/')
        .next()
        .filter(|s| !s.is_empty());

    let stem = if let Some(segment) = base {
        let decoded = urlencoding::decode(segment).unwrap_or_else(|_| segment.into());
        sanitize_filename(&decoded)
    } else {
        format!("wallpaper_{}", chrono::Utc::now().timestamp())
    };

    // Split the sanitised stem into base name and existing extension.
    let (base_stem, existing_ext) = match parse_ext_from_filename(&stem) {
        Some(ext) => {
            let dot_pos = stem.rfind('.').unwrap();
            (&stem[..dot_pos], Some(ext))
        }
        None => (stem.as_str(), None),
    };

    let needs_detected_ext = match existing_ext {
        None => true,
        Some(ext) => {
            !crate::utils::is_known_ext(ext) || normalize_ext(ext) != normalize_ext(detected_ext)
        }
    };

    if needs_detected_ext {
        format!("{base_stem}.{detected_ext}")
    } else {
        stem
    }
}

/// Sanitizes a string for use as a filename.
pub fn sanitize_filename(name: &str) -> String {
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

/// Validates that raw bytes represent a supported image format.
pub fn validate_image(bytes: &[u8]) -> Result<(), String> {
    if bytes.is_empty() {
        return Err("Empty response body".into());
    }
    let is_image = bytes.starts_with(b"\xFF\xD8\xFF")
        || bytes.starts_with(b"\x89PNG\r\n\x1A\n")
        || bytes.starts_with(b"RIFF") && bytes.len() > 8
        || bytes.starts_with(b"BM")
        || bytes.starts_with(b"GIF8");

    if is_image {
        Ok(())
    } else {
        Err("Downloaded file does not appear to be an image".into())
    }
}

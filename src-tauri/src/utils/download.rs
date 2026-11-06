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

/// Derives a safe filename from a URL, using the Content-Type and magic bytes
/// to determine the correct extension.
pub fn derive_filename(url: &str, content_type: &str, bytes: &[u8]) -> String {
    let ext = if !content_type.is_empty() {
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

    format!("{stem}.{ext}")
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

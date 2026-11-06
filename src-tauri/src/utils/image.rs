use base64::Engine;

/// Maps a file extension to its MIME type.
pub fn ext_to_mime(ext: &str) -> &str {
    match ext {
        "png" => "image/png",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        "gif" => "image/gif",
        _ => "image/jpeg",
    }
}

/// Encodes raw bytes as a base64 data URL.
pub fn encode_data_url(bytes: &[u8], mime: &str) -> String {
    let b64 = base64::engine::general_purpose::STANDARD.encode(bytes);
    format!("data:{mime};base64,{b64}")
}

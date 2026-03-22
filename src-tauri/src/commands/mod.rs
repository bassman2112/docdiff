use crate::extractors;
use std::path::Path;

#[tauri::command]
pub fn extract_text(file_path: String) -> Result<Vec<String>, String> {
    let path = Path::new(&file_path);

    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| "File has no extension".to_string())?;

    let bytes = std::fs::read(path).map_err(|e| format!("Failed to read file: {e}"))?;

    let extractor = extractors::get_extractor(extension)?;
    extractor.extract(&bytes)
}

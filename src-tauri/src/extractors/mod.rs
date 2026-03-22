pub mod docx;
pub mod rtf;
pub mod txt;

pub trait TextExtractor {
    fn extract(&self, bytes: &[u8]) -> Result<Vec<String>, String>;
}

pub fn get_extractor(extension: &str) -> Result<Box<dyn TextExtractor>, String> {
    match extension.to_lowercase().as_str() {
        "docx" => Ok(Box::new(docx::DocxExtractor)),
        "rtf" => Ok(Box::new(rtf::RtfExtractor)),
        "txt" | "text" | "md" | "csv" | "json" | "xml" | "html" | "htm" | "log" | "yaml"
        | "yml" | "toml" | "cfg" | "ini" | "rst" => Ok(Box::new(txt::TxtExtractor)),
        ext => Err(format!("Unsupported file type: .{ext}")),
    }
}

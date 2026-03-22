use super::TextExtractor;

pub struct TxtExtractor;

impl TextExtractor for TxtExtractor {
    fn extract(&self, bytes: &[u8]) -> Result<Vec<String>, String> {
        let content =
            String::from_utf8(bytes.to_vec()).map_err(|e| format!("Invalid UTF-8: {e}"))?;
        Ok(content.lines().map(|l| l.to_string()).collect())
    }
}

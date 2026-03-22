use super::TextExtractor;
use docx_rs::*;

pub struct DocxExtractor;

impl TextExtractor for DocxExtractor {
    fn extract(&self, bytes: &[u8]) -> Result<Vec<String>, String> {
        let doc = read_docx(bytes).map_err(|e| format!("Failed to read .docx file: {e}"))?;

        let mut paragraphs = Vec::new();

        for child in doc.document.children {
            match child {
                DocumentChild::Paragraph(p) => {
                    let text = extract_paragraph_text(&p);
                    paragraphs.push(text);
                }
                DocumentChild::Table(t) => {
                    for row in &t.rows {
                        match row {
                            TableChild::TableRow(tr) => {
                                let cells: Vec<String> = tr
                                    .cells
                                    .iter()
                                    .map(|cell| {
                                        let TableRowChild::TableCell(tc) = cell;
                                        extract_table_cell_text(tc)
                                    })
                                    .collect();
                                paragraphs.push(format!("| {} |", cells.join(" | ")));
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(paragraphs)
    }
}

fn extract_paragraph_text(paragraph: &Paragraph) -> String {
    let mut text = String::new();
    for child in &paragraph.children {
        if let ParagraphChild::Run(run) = child {
            for run_child in &run.children {
                match run_child {
                    RunChild::Text(t) => text.push_str(&t.text),
                    RunChild::Tab(_) => text.push('\t'),
                    RunChild::Break(_) => text.push('\n'),
                    _ => {}
                }
            }
        }
    }
    text
}

fn extract_table_cell_text(cell: &TableCell) -> String {
    let mut parts = Vec::new();
    for child in &cell.children {
        if let TableCellContent::Paragraph(p) = child {
            let text = extract_paragraph_text(p);
            if !text.is_empty() {
                parts.push(text);
            }
        }
    }
    parts.join(" ")
}

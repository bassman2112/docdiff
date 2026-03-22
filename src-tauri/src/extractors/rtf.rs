use super::TextExtractor;

pub struct RtfExtractor;

impl TextExtractor for RtfExtractor {
    fn extract(&self, bytes: &[u8]) -> Result<Vec<String>, String> {
        let raw = String::from_utf8(bytes.to_vec()).map_err(|e| format!("Invalid UTF-8: {e}"))?;
        let text = strip_rtf(&raw);
        Ok(text.lines().map(|l| l.to_string()).collect())
    }
}

/// Simple RTF-to-plaintext stripper.
fn strip_rtf(input: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut skip_depth: Option<usize> = None;
    let mut depth: usize = 0;

    while i < len {
        let ch = chars[i];
        match ch {
            '{' => {
                depth += 1;
                // Check for \* destination group → skip it entirely
                if i + 2 < len && chars[i + 1] == '\\' && chars[i + 2] == '*' {
                    if skip_depth.is_none() {
                        skip_depth = Some(depth);
                    }
                }
                i += 1;
            }
            '}' => {
                if skip_depth == Some(depth) {
                    skip_depth = None;
                }
                depth = depth.saturating_sub(1);
                i += 1;
            }
            '\\' if skip_depth.is_some() => {
                // Inside a skipped destination — advance past control word
                i += 1;
                skip_control_word(&chars, &mut i);
            }
            '\\' => {
                i += 1;
                if i >= len {
                    break;
                }
                match chars[i] {
                    // Hex escape: \'XX
                    '\'' => {
                        i += 1;
                        if i + 1 < len {
                            let hex: String = chars[i..i + 2].iter().collect();
                            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                                out.push(byte as char);
                            }
                            i += 2;
                        }
                    }
                    // Newline control words
                    'p' if matches_word(&chars, i, "par") => {
                        i += 3;
                        skip_trailing_space(&chars, &mut i);
                        out.push('\n');
                    }
                    'l' if matches_word(&chars, i, "line") => {
                        i += 4;
                        skip_trailing_space(&chars, &mut i);
                        out.push('\n');
                    }
                    't' if matches_word(&chars, i, "tab") => {
                        i += 3;
                        skip_trailing_space(&chars, &mut i);
                        out.push('\t');
                    }
                    // Escaped literal characters
                    '\\' | '{' | '}' => {
                        out.push(chars[i]);
                        i += 1;
                    }
                    '\n' | '\r' => {
                        // Escaped newline — treat as \par
                        i += 1;
                        out.push('\n');
                    }
                    // Any other control word — skip it
                    _ => {
                        skip_control_word(&chars, &mut i);
                    }
                }
            }
            '\n' | '\r' => {
                // Bare newlines in RTF are ignored
                i += 1;
            }
            _ => {
                if skip_depth.is_none() {
                    out.push(ch);
                }
                i += 1;
            }
        }
    }

    out
}

/// Advance `i` past an RTF control word (letters optionally followed by a number
/// and a single trailing space delimiter).
fn skip_control_word(chars: &[char], i: &mut usize) {
    let len = chars.len();
    // Skip alphabetic control word
    while *i < len && chars[*i].is_ascii_alphabetic() {
        *i += 1;
    }
    // Skip optional numeric parameter (including leading minus)
    if *i < len && (chars[*i] == '-' || chars[*i].is_ascii_digit()) {
        if chars[*i] == '-' {
            *i += 1;
        }
        while *i < len && chars[*i].is_ascii_digit() {
            *i += 1;
        }
    }
    // Skip single trailing space delimiter
    if *i < len && chars[*i] == ' ' {
        *i += 1;
    }
}

/// Check if the characters at position `start` begin with the given word,
/// followed by a non-alpha character (word boundary).
fn matches_word(chars: &[char], start: usize, word: &str) -> bool {
    let wchars: Vec<char> = word.chars().collect();
    if start + wchars.len() > chars.len() {
        return false;
    }
    for (j, &wc) in wchars.iter().enumerate() {
        if chars[start + j] != wc {
            return false;
        }
    }
    // Ensure word boundary — next char is not alphabetic
    let after = start + wchars.len();
    after >= chars.len() || !chars[after].is_ascii_alphabetic()
}

fn skip_trailing_space(chars: &[char], i: &mut usize) {
    if *i < chars.len() && chars[*i] == ' ' {
        *i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_roundtrip() {
        let rtf = r"{\rtf1\ansi Hello world}";
        let result = strip_rtf(rtf);
        assert_eq!(result.trim(), "Hello world");
    }

    #[test]
    fn hex_escape() {
        let rtf = r"{\rtf1 caf\'e9}";
        let result = strip_rtf(rtf);
        assert!(result.contains("caf\u{e9}"));
    }

    #[test]
    fn par_newline() {
        let rtf = r"{\rtf1 Line one\par Line two}";
        let result = strip_rtf(rtf);
        assert!(result.contains("Line one\nLine two"));
    }

    #[test]
    fn skips_destination_groups() {
        let rtf = r"{\rtf1 Hello {\*\fonttbl hidden stuff} world}";
        let result = strip_rtf(rtf);
        assert!(!result.contains("hidden"));
        assert!(result.contains("Hello"));
        assert!(result.contains("world"));
    }

    #[test]
    fn escaped_braces() {
        let rtf = r"{\rtf1 a\{b\}c}";
        let result = strip_rtf(rtf);
        assert!(result.contains("a{b}c"));
    }
}

use crate::symbol::SYMBOL;

pub struct FilterString;

impl FilterString {
    pub fn is_valid_input_string(path: &str) -> Result<(), String> {
        let path = path.replace(|x : char | x.is_ascii_control(), SYMBOL.empty_string).trim().to_string();

        if path.contains(SYMBOL.whitespace) ||
            path.contains(SYMBOL.single_quote) ||
            path.contains(SYMBOL.quotation_mark) ||
            path.contains(SYMBOL.ampersand) ||
            path.contains(SYMBOL.pipe) ||
            path.contains(SYMBOL.semicolon) {
            return Err(format!("Path contains not allowed characters: whitespace, single quote, quotation mark, ampersand, pipe, semicolon. Path: {}",path))
        }

        Ok(())
    }
}
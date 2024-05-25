use log::{debug, warn};

/// Clean the squitter from any non-hexadecimal characters
///
/// # Arguments
///
/// * `line` - A string slice that holds the squitter
///
/// # Returns
///
/// * `Option<String>` - A cleaned squitter
///
/// # Example
///
/// ```
/// use squitterator::adsb::clean_squitter;
/// let line = "8D40621D58C382D690C8AC2863A7";
/// if let Some(result) = clean_squitter(line) {
///    assert_eq!(result, "8D40621D58C382D690C8AC2863A7");
/// }
/// ```
///
/// ```
/// use squitterator::adsb::clean_squitter;
/// let line = "@05FFD0CF94E28D49329099115719707811B06CF5;";
/// if let Some(result) = clean_squitter(line) {
///   assert_eq!(result, "8D49329099115719707811B06CF5");
/// }
/// ```
pub fn clean_squitter(line: &str) -> Option<String> {
    let ascii_line = line
        .as_bytes()
        .iter()
        .filter(|c| c.is_ascii_hexdigit())
        .map(|c| *c as char)
        .collect::<String>();
    let trimmed_line = ascii_line
        .trim()
        .trim_start_matches('@')
        .trim_end_matches(';')
        .trim();
    debug!("a_line: {}, len:{}", line, line.len());
    debug!("t_line: {}, len:{}", trimmed_line, trimmed_line.len());
    match trimmed_line.len() {
        14 | 28 => Some(trimmed_line.to_string()),
        26 | 40 => Some(trimmed_line[12..].to_string()),
        _ => {
            warn!("Invalid squitter: {}", line);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_squitter() {
        let line = "8D40621D58C382D690C8AC2863A7";
        // let format = None;
        if let Some(result) = clean_squitter(line) {
            assert_eq!(result, "8D40621D58C382D690C8AC2863A7");
        }
    }

    #[test]
    fn test_sbs_squitter() {
        let line = "@05FFD0CF94E28D49329099115719707811B06CF5;";
        // let format = None;
        if let Some(result) = clean_squitter(line) {
            assert_eq!(result, "8D49329099115719707811B06CF5");
        }
    }
}

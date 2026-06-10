use wasm_bindgen::prelude::*;

/// Returns a greeting string for `name`.
///
/// `name` is sanitized by stripping Unicode bidirectional override characters
/// (`U+202A`–`U+202E`, `U+2066`–`U+2069`) before display to prevent visual
/// spoofing. The function behavior is fully expressed by its return type;
/// callers that want to display the greeting in a dialog can call `alert()`
/// themselves explicitly.
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    let safe_name = strip_bidi(name);
    format!("Hello, {}!", safe_name)
}

fn strip_bidi(s: &str) -> String {
    s.chars()
        .filter(|c| !matches!(c, '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}'))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_bidi_override_characters() {
        assert_eq!(strip_bidi("Alice\u{202E}evil"), "Aliceevil");
        assert_eq!(strip_bidi("\u{2066}hidden"), "hidden");
        assert_eq!(strip_bidi("normal"), "normal");
    }

    #[test]
    fn allows_regular_unicode() {
        assert_eq!(strip_bidi("こんにちは"), "こんにちは");
        assert_eq!(strip_bidi("café"), "café");
    }
}

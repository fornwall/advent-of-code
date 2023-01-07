pub fn escape_text(s: &str) -> String {
    s.replace('<', "&lt;")
}

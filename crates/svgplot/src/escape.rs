use std::borrow::Cow;

pub fn escape_xml(input: &str) -> Cow<str> {
    for (i, ch) in input.char_indices() {
        if xml_escape_char(ch).is_some() {
            let mut escaped_string = String::with_capacity(input.len());
            escaped_string.push_str(&input[..i]);
            for ch in input[i..].chars() {
                match xml_escape_char(ch) {
                    Some(escaped_char) => escaped_string.push_str(escaped_char),
                    None => escaped_string.push(ch),
                };
            }
            return Cow::Owned(escaped_string);
        }
    }
    Cow::Borrowed(input)
}

const fn xml_escape_char(ch: char) -> Option<&'static str> {
    match ch {
        '&' => Some("&amp"),
        '<' => Some("&lt;"),
        '>' => Some("&gt;"),
        '"' => Some("&quot;"),
        _ => None,
    }
}

#[test]
fn hello_world_string_is_not_escaped() {
    let input = "Hello, world!";
    let escaped = escape_xml(input);
    assert_eq!(escaped, Cow::Borrowed(input));
}

#[test]
fn escapes_text() {
    let input = "This is a <script>alert('nasty');</script> string";
    let expected: Cow<str> =
        Cow::Owned("This is a &lt;script&gt;alert('nasty');&lt;/script&gt; string".to_string());
    let escaped = escape_xml(input);
    assert_eq!(escaped, expected);
}

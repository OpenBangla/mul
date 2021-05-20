
pub fn noun_stemmer(input: &str) -> String {
    let mut buffer = input.to_owned();

    if buffer.ends_with("তে") || buffer.ends_with("কে") {
        buffer.pop();
        buffer.pop();
    }

    if buffer.ends_with("রা") {
        buffer.pop();
        buffer.pop();
    }

    if buffer.ends_with('র') {
        buffer.pop();
    }

    if  buffer.ends_with('ে') && !matches!(buffer.get(buffer.len()-6..), Some("দে") | Some("কে")) {
        buffer.pop();
    }

    if buffer.ends_with('য়') {
        buffer.pop();
    }

    if buffer.ends_with("েরা") {
        buffer.pop();
        buffer.pop();
        buffer.pop();
    }

    if buffer.ends_with("দে") || buffer.ends_with("কে") || buffer.ends_with("কা") || buffer.ends_with("টা") || buffer.ends_with("টি") {
        buffer.pop();
        buffer.pop();
    }

    if buffer.ends_with("জন") || buffer.ends_with("লি") {
        buffer.pop();
        buffer.pop();
    }

    if buffer.ends_with("গুলো") || buffer.ends_with("খানা") {
        buffer.pop();
        buffer.pop();
        buffer.pop();
        buffer.pop();
    }

    buffer
}

#[cfg(test)]
mod tests {
    use super::noun_stemmer;

    #[test]
    fn test_noun_stemming() {
        assert_eq!(noun_stemmer("মানুষদেরকে"), "মানুষ");
        assert_eq!(noun_stemmer("গাছগুলোতে"), "গাছ");
        assert_eq!(noun_stemmer("বাসাতে"), "বাসা");
        assert_eq!(noun_stemmer("মানুষকে"), "মানুষ");
        assert_eq!(noun_stemmer("মার"), "মা");
        assert_eq!(noun_stemmer("বাবার"), "বাবা");
        assert_eq!(noun_stemmer("মানুষটির"), "মানুষ");
        assert_eq!(noun_stemmer("মানুষের"), "মানুষ");
        assert_eq!(noun_stemmer("মানুষদের"), "মানুষ");
        assert_eq!(noun_stemmer("আজকের"), "আজ");
        assert_eq!(noun_stemmer("মানুষজন"), "মানুষ");
        assert_eq!(noun_stemmer("এখানকার"), "এখান");
        assert_eq!(noun_stemmer("মাছের"), "মাছ");
        assert_eq!(noun_stemmer("বইয়ে"), "বই");
        assert_eq!(noun_stemmer("মানুষেরা"), "মানুষ");
    }
}

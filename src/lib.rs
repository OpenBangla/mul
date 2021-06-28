pub fn noun_stemmer(input: &str) -> String {
    let mut buffer = input.to_owned();
    let mut remove_y = true;

    if buffer.ends_with("তে") || buffer.ends_with("কে") {
        string_pop(&mut buffer, 2);
    }

    if buffer.ends_with("রা") {
        string_pop(&mut buffer, 2);
    }

    if buffer.ends_with("য়ের") {
        if noun_eliminate_y(&buffer[..buffer.len() - 9]) {
            string_pop(&mut buffer, 3);
        } else {
            // Just remove ে and র, and mark য় to not to be removed.
            string_pop(&mut buffer, 2);
            remove_y = false;
        }
    }

    if buffer.ends_with('র') {
        buffer.pop();
    }

    if buffer.ends_with('ে') && !matches!(buffer.get(buffer.len() - 6..), Some("দে") | Some("কে"))
    {
        buffer.pop();
    }

    if buffer.ends_with('য়') && remove_y {
        buffer.pop();
    }

    if buffer.ends_with("েরা") {
        string_pop(&mut buffer, 3);
    }

    if buffer.ends_with("টি") {
        // Check if the `ট` character is a part of a jukktakkhor.
        if let Some(c) = buffer
            .get(..buffer.len() - 6)
            .and_then(|s| s.get(s.len() - 3..))
        {
            if c != "্" {
                string_pop(&mut buffer, 2);
            }
        }
    }

    if buffer.ends_with("দে")
        || buffer.ends_with("কে")
        || buffer.ends_with("কা")
        || buffer.ends_with("টা")
    {
        string_pop(&mut buffer, 2);
    }

    if buffer.ends_with("জন") || buffer.ends_with("লি") {
        string_pop(&mut buffer, 2);
    }

    if buffer.ends_with("গুলো") || buffer.ends_with("খানা") {
        string_pop(&mut buffer, 4);
    }

    buffer
}

/// Checks if the character `য়` is removable in the `term` word.
/// The `term` word is all the characters up to the `য়` character of a word.
///
/// It is removable if the length of the _stem_ of the term word is one and
/// the last character is a vowel.
///
/// **Stem**: It is the word without any vowel signs.
fn noun_eliminate_y(term: &str) -> bool {
    let stem_len = term
        .chars()
        .filter(|c| {
            match c {
                // Vowel signs
                '\u{09BE}'..='\u{09C8}' => false,
                _ => true,
            }
        })
        .count();

    stem_len == 1 || is_vowel(&term[term.len() - 3..])
}

/// Checks if the `c` is a Bengali vowel character.
fn is_vowel(c: &str) -> bool {
    match c.chars().next().unwrap() {
        '\u{0985}'..='\u{0994}' => true,
        _ => false,
    }
}

/// Removes last `n` characters from the `string` buffer.
fn string_pop(string: &mut String, n: usize) {
    let new_len = string.len() - n * 3; // Every Bengali character is of 3 Bytes.
    string.truncate(new_len);
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
        // Special case 1
        assert_eq!(noun_stemmer("মায়ের"), "মা");
        assert_eq!(noun_stemmer("বইয়ের"), "বই");
        assert_eq!(noun_stemmer("পায়ের"), "পা");
        assert_eq!(noun_stemmer("ভাইয়ের"), "ভাই");
        assert_eq!(noun_stemmer("বউয়ের"), "বউ");
        assert_eq!(noun_stemmer("উভয়ের"), "উভয়");
        // Special case 2
        assert_eq!(noun_stemmer("মেষটির"), "মেষ");
        assert_eq!(noun_stemmer("বৃষ্টির"), "বৃষ্টি");
    }
}

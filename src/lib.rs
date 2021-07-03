//! A Bengali stemmer
//!
//! `mul` currently implements a stepwise approach to removing inflections from Bengali words <sup>[1](#references)</sup>.
//!
//! ## Example
//! ```
//! use mul::noun_stemmer;
//!
//! fn main() {
//!     assert_eq!(noun_stemmer("বাংলায়"), "বাংলা");
//!     assert_eq!(noun_stemmer("মানুষকে"), "মানুষ");
//! }
//! ```
//!
//! ## References
//! 1. M. R. Mahmud, M. Afrin, M. A. Razzaque, E. Miller and J. Iwashige, "A rule based bengali stemmer,"
//! _2014 International Conference on Advances in Computing, Communications and Informatics (ICACCI)_,
//! 2014, pp. 2750-2756, doi: 10.1109/ICACCI.2014.6968484.

/// Removes noun inflections such as `টি`, `কে`, `তে` etc from the `input` and returns it.
///
/// ```
/// use mul::noun_stemmer;
/// 
/// # fn main() {
/// assert_eq!(noun_stemmer("মানুষদেরকে"), "মানুষ");
/// assert_eq!(noun_stemmer("গাছগুলোতে"), "গাছ");
/// assert_eq!(noun_stemmer("বাসাতে"), "বাসা");
/// # }
/// ```
pub fn noun_stemmer(input: &str) -> String {
    let mut buffer = input.to_owned();
    let mut remove_y = true;

    // Remove ই only if the length of the stem of the word without ই is more than one character.
    if buffer.ends_with("ই") && stem_len(&buffer[..buffer.len() - 3]) != 1 {
        buffer.pop();
    }

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

    // Remove র only if it has a preceding kar character.
    if buffer.ends_with('র') && is_kar(&buffer[..buffer.len() - 3][buffer.len() - 6..]) {
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
/// **Stem**: The word without any vowel signs.
fn noun_eliminate_y(term: &str) -> bool {
    stem_len(term) == 1 || is_vowel(&term[term.len() - 3..])
}

/// Returns the length of the `term` word without any Kars or vowel signs.
fn stem_len(term: &str) -> usize {
    term.chars()
        .filter(|c| {
            match c {
                // Vowel signs
                '\u{09BE}'..='\u{09C8}' => false,
                _ => true,
            }
        })
        .count()
}

/// Checks if the `c` is a Bengali vowel character.
fn is_vowel(c: &str) -> bool {
    match c.chars().next() {
        Some('\u{0985}'..='\u{0994}') => true,
        _ => false,
    }
}

fn is_kar(c: &str) -> bool {
    match c.chars().next() {
        Some('\u{09BE}'..='\u{09C8}') => true,
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
        assert_eq!(noun_stemmer("মানুষটির"), "মানুষ");
        assert_eq!(noun_stemmer("মানুষের"), "মানুষ");
        assert_eq!(noun_stemmer("মানুষদের"), "মানুষ");
        assert_eq!(noun_stemmer("আজকের"), "আজ");
        assert_eq!(noun_stemmer("মানুষজন"), "মানুষ");
        assert_eq!(noun_stemmer("এখানকার"), "এখান");
        assert_eq!(noun_stemmer("মাছের"), "মাছ");
        assert_eq!(noun_stemmer("মানুষেরা"), "মানুষ");
        assert_eq!(noun_stemmer("ঐক্যের"), "ঐক্য");
        assert_eq!(noun_stemmer("ওয়ার্নারের"), "ওয়ার্নার");
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
        // Special case 3
        assert_eq!(noun_stemmer("মার"), "মা");
        assert_eq!(noun_stemmer("বাবার"), "বাবা");
        assert_eq!(noun_stemmer("গরুর"), "গরু");
        assert_eq!(noun_stemmer("মমতার"), "মমতা");
        assert_eq!(noun_stemmer("পাথর"), "পাথর");
        assert_eq!(noun_stemmer("শুক্র"), "শুক্র");
        // Special case 3
        assert_eq!(noun_stemmer("বই"), "বই");
        assert_eq!(noun_stemmer("লুই"), "লুই");
        assert_eq!(noun_stemmer("সমাধানই"), "সমাধান");
        assert_eq!(noun_stemmer("সহজেই"), "সহজ");
    }
}

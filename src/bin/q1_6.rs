use std::{collections::BTreeMap, fmt::format};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_compression() {
        assert_eq!(string_compression("aabcccccaaa"), "a2b1c5a3");
        assert_eq!(string_compression("abc"), "a1b1c1");
        assert_eq!(string_compression("aabbcc"), "a2b2c2");
        assert_eq!(string_compression("aaabbbccc"), "a3b3c3");
        assert_eq!(string_compression(""), "");
    }
}

fn count_chars(s: &str) -> BTreeMap<i32, (char, i32)> {
    let mut char_count: BTreeMap<i32, (char, i32)> = BTreeMap::new();
    let mut last_char: Option<char> = None;
    let mut index = 0;
    for c in s.chars() {
        match last_char {
            Some(ch) if ch == c => {
                if let Some((_, count)) = char_count.get_mut(&index) {
                    *count += 1;
                }
            }
            _ => {
                index += 1;
                char_count.insert(index, (c, 1));
                last_char = Some(c);
            }
        }
    }
    char_count
}


fn string_compression(string: &str) -> String {
    let mut return_string = String::new();
    let counted_chars = count_chars(string);
    for (key, (char, value) ) in counted_chars {
        println!("{}: {}", key, value);
        return_string.push_str(&format!("{}{}", char, value));
    }
    println!("{}",return_string);
    return_string
}

fn main() {
    string_compression("aabcccccaaa");
}

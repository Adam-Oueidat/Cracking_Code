fn one_away(string1: &str, string2: &str) -> bool {
    let mut no_of_edits = 0;
    if string1 == string2 {
        return true;
    }
    if (string1.len() as i32 - string2.len() as i32).abs() > 1 {
        return false;
    }
    if string1.len() == string2.len() {
        return one_char_changed(string1, string2);
    }
    if string2.len() > string1.len() {
        return one_char_inserted(string2, string1);
    }

    return one_char_inserted(string1, string2);
}

fn one_char_inserted(s1: &str, s2: &str) -> bool {
    assert!(s1.len() == s2.len() + 1);
    let mut idx1 = 0;
    let mut idx2 = 0;
    while idx1 < s1.len() && idx2 < s2.len() {
        let char1 = s1.get(idx1..=idx1).unwrap();
        let char2 = s2.get(idx2..=idx2).unwrap();
        if char1 == char2 {
            idx2 += 1;
        }
        idx1 += 1;
    }
    idx1 >= s1.len() - 1 && idx2 == s2.len()
}

fn one_char_changed(string1: &str, string2: &str) -> bool {
    let mut had_edit = false;
    for (c1, c2) in string1.chars().zip(string2.chars()) {
        if c1 != c2 && !had_edit {
            had_edit = true;
        } else if c1 != c2 {
            return false;
        }
    }
    true
}

mod tests {
    use super::*;

    #[test]
    fn test_one_away() {
        assert_eq!(one_away("pale", "ple"), true);
        assert_eq!(one_away("pales", "pale"), true);
        assert_eq!(one_away("pale", "bale"), true);
        assert_eq!(one_away("pale", "bake"), false);
    }
}

fn main() {
    one_away("pale", "ple");
}

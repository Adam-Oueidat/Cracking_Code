use std::collections::HashMap;


fn palindrome_permutation(string: &str) -> bool {
    let removed_spaces_string = string.replace(" ", "");
    let characters = count_chars(&removed_spaces_string);
    let is_even = removed_spaces_string.len() % 2 == 0;
    let mut is_odd = false;

    for value in characters.values(){
        if value % 2 != 0 {
            if is_even || is_odd {
                return false;
            }
            else {
                is_odd = true;
            }
        }
    }
    
    return true;
}
fn count_chars(s: &str) -> HashMap<char, i32> {
    let mut char_count = HashMap::new();
    let removed_spaces_s = s.replace(" ", "");
    for c in removed_spaces_s.chars() {
        let lower_c = c.to_lowercase().next().unwrap();
        let count = char_count.entry(lower_c).or_insert(0);
        *count += 1;
    }
    char_count
}



mod tests {
    use super::*;

    #[test]
    fn test_is_permutation() {
        assert_eq!(palindrome_permutation("Tact Coa"), true);
        assert_eq!(palindrome_permutation("Able was I ere I saw Elba"), true);
        assert_eq!(palindrome_permutation("abcdcbda") , true);
        assert_eq!(palindrome_permutation("a") , true);


    }
    #[test]
    fn test_is_not_palindrome(){
        // Tests for non-palindromes
        assert_eq!(palindrome_permutation("Not a palindrome"), false);
        assert_eq!(palindrome_permutation("Also not a palindrome"), false);
        assert_eq!(palindrome_permutation("Definitely not a palindrome"), false);
    }
}

fn main(){
    palindrome_permutation("Tact Coa");
}
// Check Permutation: Given two strings,write a method to decide if one is a permutation of the
// other.
use std::collections::HashMap;

fn count_chars(s: &str) -> HashMap<char, i32> {
    let mut char_count = HashMap::new();
    for c in s.chars() {
        let count = char_count.entry(c).or_insert(0);
        *count += 1;
    }
    char_count
}

fn is_permutation(string1: &str, string2: &str) -> bool {
    if string1.len() != string2.len() {
        return false;
    }

    let characters_from_string1 = count_chars(string1);
    let characters_from_string2 = count_chars(string2);

    for key in characters_from_string2.keys() {
        if !characters_from_string1.contains_key(key){
            return false;
        }
        if characters_from_string1.get(key) != characters_from_string2.get(key){
            return false
        }

    for key in characters_from_string1.keys(){
        if !characters_from_string2.contains_key(key){
            return false;
        }
    }

    }
    return true;
    

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_permutation() {
        assert_eq!(
            is_permutation(&String::from("cat"), &String::from("tac")),
            true
        );
        
    }
    #[test]
    fn test_is_not_permutation(){
        assert_eq!(
            is_permutation(&String::from("cat"), &String::from("dog")),
            false
        );
    }
}

fn main() {
    is_permutation(&String::from("cat"), &String::from("dog"));
}
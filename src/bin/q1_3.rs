fn replace_space(s: &str, length: usize) -> String {
    s.trim().replace(' ', "%20")
}

fn urlify(url: &'static str) -> String {
    let placeholder = "%20";

    url.split_whitespace().fold(String::new(), |acc, s| {
        if acc.is_empty() {
            String::from(s)
        } else {
            acc + placeholder + s
        }
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_permutation() {
        assert_eq!(replace_space("Mr John Smith ", 13), "Mr%20John%20Smith");
    }
}

fn main() {
    replace_space("Mr John Smith ", 13);
}

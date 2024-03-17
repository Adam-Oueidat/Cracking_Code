pub fn main() {
    let string1 = "abcdef"; // unique characters
    let string2 = "ghijkl"; // unique characters
    let string3 = "mnopqr"; // unique characters
    let string4 = "stuvwx"; // unique characters
    let string5 = "yz0123"; // unique characters
    let string6 = "aaaaaa"; // not unique characters
    let string7 = "bbbbbb"; // not unique characters
    let string8 = "cccccc"; // not unique characters
    let string9 = "dddddd"; // not unique characters
    let string10 = "eeeeee"; // not unique characters
    
    println!("{}", is_unique(string1));
    println!("{}", is_unique(string2));
    println!("{}", is_unique(string3));
    println!("{}", is_unique(string4));
    println!("{}", is_unique(string5));
    println!("{}", is_unique(string6));
    println!("{}", is_unique(string7));
    println!("{}", is_unique(string8));
    println!("{}", is_unique(string9));
    println!("{}", is_unique(string10));
}

fn is_unique(string: &str) -> bool {
    let mut char_set = [false; 128];
    let number_of_characters = get_number_of_characters();
    if string.len() > number_of_characters {
        return false;
    }

    for c in string.chars() {
        let index = c as usize;
        if char_set[index] {
            return false;
        }
        char_set[index] = true;
    }
    return true
}

fn get_number_of_characters() -> usize {
    // Depends on what type of characters
    return 28;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        assert_eq!(is_unique(&String::from("abcdefg")), true);
        assert_eq!(is_unique(&String::from("abcdefga")), false);
    }

}
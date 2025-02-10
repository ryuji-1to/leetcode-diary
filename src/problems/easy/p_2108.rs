pub fn first_palindrome(words: Vec<String>) -> String {
    for v in words {
        let v_rev = v.chars().rev().collect::<String>();
        if v == v_rev {
            return v;
        }
    }
    "".to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            "ada".to_string(),
            first_palindrome(vec![
                "abc".to_string(),
                "car".to_string(),
                "ada".to_string(),
                "racecar".to_string(),
                "cool".to_string()
            ])
        );
        assert_eq!(
            "racecar".to_string(),
            first_palindrome(vec!["notapalindrome".to_string(), "racecar".to_string(),])
        );
        assert_eq!(
            "".to_string(),
            first_palindrome(vec!["def".to_string(), "ghi".to_string(),])
        )
    }
}

pub fn remove_palindrome_sum(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    }
    if s == s.chars().rev().collect::<String>() {
        return 1;
    }
    2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, remove_palindrome_sum("ababa".to_string()));
        assert_eq!(2, remove_palindrome_sum("abb".to_string()));
        assert_eq!(2, remove_palindrome_sum("baabb".to_string()));
    }
}

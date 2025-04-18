pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
    words.iter().filter(|&x| s.starts_with(x)).count() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            3,
            count_prefixes(
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "ab".to_string(),
                    "bc".to_string(),
                    "abc".to_string()
                ],
                "abc".to_string()
            )
        );
        assert_eq!(
            2,
            count_prefixes(vec!["a".to_string(), "a".to_string(),], "aa".to_string())
        );
    }
}

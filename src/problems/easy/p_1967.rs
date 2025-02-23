pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
    patterns
        .iter()
        .fold(0, |acc, x| if word.contains(x) { acc + 1 } else { acc })
}

#[cfg(test)]
mod test {
    use std::num;

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            3,
            num_of_strings(
                vec![
                    "a".to_string(),
                    "abc".to_string(),
                    "bc".to_string(),
                    "d".to_string()
                ],
                "abc".to_string()
            )
        );
        assert_eq!(
            2,
            num_of_strings(
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
                "aaaaabbbbb".to_string()
            )
        );
        assert_eq!(
            3,
            num_of_strings(
                vec!["a".to_string(), "a".to_string(), "a".to_string(),],
                "ab".to_string()
            )
        );
    }
}

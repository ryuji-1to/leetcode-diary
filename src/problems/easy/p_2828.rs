pub fn is_acronym(words: Vec<String>, s: String) -> bool {
    // if words.len() != s.chars().count() {
    //     return false;
    // }
    // s.chars().zip(words).all(|(x, y)| y.starts_with(x))

    words
        .iter()
        .map(|x| x.chars().next().unwrap())
        .collect::<String>()
        == s
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            true,
            is_acronym(
                vec![
                    "alice".to_string(),
                    "bob".to_string(),
                    "charlie".to_string()
                ],
                "abc".to_string()
            )
        );
        assert_eq!(
            false,
            is_acronym(
                vec!["an".to_string(), "apple".to_string(),],
                "a".to_string()
            )
        );
        assert_eq!(
            true,
            is_acronym(
                vec![
                    "never".to_string(),
                    "gonna".to_string(),
                    "give".to_string(),
                    "up".to_string(),
                    "on".to_string(),
                    "you".to_string()
                ],
                "ngguoy".to_string()
            )
        );
    }
}

pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
    words
        .iter()
        .flat_map(|x| x.split(separator))
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![
                "one".to_string(),
                "two".to_string(),
                "three".to_string(),
                "four".to_string(),
                "five".to_string(),
                "six".to_string()
            ],
            split_words_by_separator(
                vec!["one.two.three".into(), "four.five".into(), "six".into()],
                '.'
            )
        );
        assert_eq!(
            vec!["easy".to_string(), "problem".to_string(),],
            split_words_by_separator(vec!["$easy$".into(), "$problem$".into()], '$')
        );
        assert_eq!(
            vec![] as Vec<String>,
            split_words_by_separator(vec!["|||".to_string()], '|')
        );
    }
}

pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
    let target = if words1.len() > words2.len() {
        &words2
    } else {
        &words1
    };
    let mut map1 = std::collections::HashMap::new();
    for w in &words1 {
        *map1.entry(w).or_insert(0) += 1;
    }
    let mut map2 = std::collections::HashMap::new();
    for w in &words2 {
        *map2.entry(w).or_insert(0) += 1;
    }

    target
        .iter()
        .filter(|&s| map1.get(s) == Some(&1) && map2.get(s) == Some(&1))
        .count() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            2,
            count_words(
                vec![
                    "leetcode".to_string(),
                    "is".to_string(),
                    "amazing".to_string(),
                    "as".to_string(),
                    "is".to_string()
                ],
                vec![
                    "amazing".to_string(),
                    "leetcode".to_string(),
                    "is".to_string()
                ]
            )
        );
        assert_eq!(
            0,
            count_words(
                vec!["b".to_string(), "bb".to_string(), "bbb".to_string(),],
                vec!["a".to_string(), "aa".to_string(), "aaa".to_string()]
            )
        );
        assert_eq!(
            1,
            count_words(
                vec!["a".to_string(), "ab".to_string(),],
                vec![
                    "a".to_string(),
                    "a".to_string(),
                    "a".to_string(),
                    "ab".to_string()
                ]
            )
        );
    }
}

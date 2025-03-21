pub fn count_prefix_suffix_pairs(mut words: Vec<String>) -> i32 {
    let mut result = 0;
    for i in 0..words.len() - 1 {
        for j in i + 1..words.len() {
            if words[j].starts_with(&words[i]) && words[j].ends_with(&words[i]) {
                result += 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            4,
            count_prefix_suffix_pairs(vec![
                "a".to_string(),
                "aba".to_string(),
                "ababa".to_string(),
                "aa".to_string()
            ])
        );
        assert_eq!(
            2,
            count_prefix_suffix_pairs(vec![
                "pa".to_string(),
                "papa".to_string(),
                "ma".to_string(),
                "ma".to_string()
            ])
        );
        assert_eq!(
            0,
            count_prefix_suffix_pairs(vec!["abab".to_string(), "ab".to_string(),])
        );
    }
}

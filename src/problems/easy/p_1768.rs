pub fn merge_alternately(word1: String, word2: String) -> String {
    let chars1: Vec<char> = word1.chars().collect();
    let chars2: Vec<char> = word2.chars().collect();
    let mut result = String::with_capacity(chars1.len() + chars2.len());

    for i in 0..chars1.len().max(chars2.len()) {
        if let Some(&v) = chars1.get(i) {
            result.push(v);
        }
        if let Some(&v) = chars2.get(i) {
            result.push(v);
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
            "apbqcr".to_string(),
            merge_alternately("abc".to_string(), "pqr".to_string())
        );
        assert_eq!(
            "apbqrs".to_string(),
            merge_alternately("ab".to_string(), "pqrs".to_string())
        );
        assert_eq!(
            "apbqcd".to_string(),
            merge_alternately("abcd".to_string(), "pq".to_string())
        );
    }
}

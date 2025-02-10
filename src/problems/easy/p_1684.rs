use std::collections::HashSet;

pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    // let result = words
    //     .iter()
    //     .filter(|x| {
    //         for v in x.chars() {
    //             if !allowed.contains(v) {
    //                 return false;
    //             }
    //         }
    //         true
    //     })
    //     .collect::<Vec<_>>();
    // result.len() as i32
    let allowed_set: HashSet<char> = allowed.chars().collect();
    words
        .iter()
        .filter(|word| word.chars().all(|c| allowed_set.contains(&c)))
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            2,
            count_consistent_strings(
                "ab".to_string(),
                vec![
                    "ad".to_string(),
                    "bd".to_string(),
                    "aaab".to_string(),
                    "baa".to_string(),
                    "badab".to_string()
                ]
            )
        );
        assert_eq!(
            7,
            count_consistent_strings(
                "abc".to_string(),
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "ab".to_string(),
                    "ac".to_string(),
                    "bc".to_string(),
                    "abc".to_string()
                ]
            )
        );
        assert_eq!(
            4,
            count_consistent_strings(
                "cad".to_string(),
                vec![
                    "cc".to_string(),
                    "acd".to_string(),
                    "b".to_string(),
                    "ba".to_string(),
                    "bac".to_string(),
                    "bad".to_string(),
                    "ac".to_string(),
                    "d".to_string()
                ]
            )
        );
    }
}

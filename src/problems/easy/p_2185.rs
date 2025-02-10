pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
    words.iter().fold(0, |acc, x| {
        if x.starts_with(pref.as_str()) {
            acc + 1
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            2,
            prefix_count(
                vec![
                    "pay".to_string(),
                    "attention".to_string(),
                    "practice".to_string(),
                    "attend".to_string()
                ],
                "at".to_string()
            )
        );
        assert_eq!(
            0,
            prefix_count(
                vec![
                    "leetcode".to_string(),
                    "win".to_string(),
                    "loops".to_string(),
                    "success".to_string()
                ],
                "code".to_string()
            )
        );
    }
}

pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
    let cs = ['a', 'e', 'i', 'o', 'u'];
    words[left as usize..=right as usize]
        .iter()
        .filter(|&c| cs.iter().any(|&x| c.starts_with(x)) && cs.iter().any(|&x| c.ends_with(x)))
        .count() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            2,
            vowel_strings(
                vec!["are".to_string(), "amy".to_string(), "u".to_string()],
                0,
                2
            )
        );
        assert_eq!(
            3,
            vowel_strings(
                vec![
                    "hey".to_string(),
                    "aeo".to_string(),
                    "mu".to_string(),
                    "ooo".to_string(),
                    "artro".to_string()
                ],
                1,
                4
            )
        );
    }
}

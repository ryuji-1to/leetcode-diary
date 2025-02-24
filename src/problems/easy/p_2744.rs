pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
    words.iter().enumerate().fold(0, |acc, (i, x)| {
        let reversed: String = x.chars().rev().collect();
        if (&words[i + 1..]).contains(&reversed) {
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
            maximum_number_of_string_pairs(vec![
                "cd".to_string(),
                "ac".to_string(),
                "dc".to_string(),
                "ca".to_string(),
                "zz".to_string()
            ])
        );
        assert_eq!(
            1,
            maximum_number_of_string_pairs(vec![
                "ab".to_string(),
                "ba".to_string(),
                "cc".to_string(),
            ])
        );
        assert_eq!(
            0,
            maximum_number_of_string_pairs(vec!["aa".to_string(), "ab".to_string(),])
        );
    }
}

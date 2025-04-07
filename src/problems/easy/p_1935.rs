pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
    // let splited = text
    //     .split_whitespace()
    //     .filter(|&x| !x.is_empty())
    //     .collect::<Vec<&str>>();
    // let original_len = splited.len() as i32;
    // let broken_len = splited.iter().fold(0, |acc, &w| {
    //     for b in broken_letters.chars() {
    //         if w.contains(b) {
    //             return acc + 1;
    //         }
    //     }
    //     acc
    // });
    // original_len - broken_len
    let broken_list = broken_letters.chars().collect::<Vec<char>>();
    text.split_whitespace()
        .filter(|&word| !broken_list.iter().any(|&b| word.contains(b)))
        .count() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            1,
            can_be_typed_words("hello world".to_string(), "ad".to_string())
        );
        assert_eq!(
            1,
            can_be_typed_words("leet code".to_string(), "lt".to_string())
        );
        assert_eq!(
            0,
            can_be_typed_words("leet code".to_string(), "e".to_string())
        );
    }
}

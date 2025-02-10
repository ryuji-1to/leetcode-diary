pub fn most_words_found(sentences: Vec<String>) -> i32 {
    sentences
        .iter()
        .map(|x| x.split_whitespace().count())
        .max()
        .unwrap_or(0) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            6,
            most_words_found(vec![
                String::from("alice and bob love leetcode"),
                String::from("i think so too"),
                String::from("this is great thanks very much")
            ])
        );
        assert_eq!(
            3,
            most_words_found(vec![
                String::from("please wait"),
                String::from("continue to fight"),
                String::from("continue to win")
            ])
        );
    }
}

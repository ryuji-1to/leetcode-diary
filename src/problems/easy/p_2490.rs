pub fn is_circular_sentence(sentence: String) -> bool {
    let words = sentence.split_whitespace().collect::<Vec<_>>();
    let mut current = "";
    for i in 0..words.len() {
        let first = &words[i][..1];
        let last = &words[i][words[i].len() - 1..];

        if current != "" && current != first {
            return false;
        }

        current = last;
    }
    current == &words[0][..1]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            true,
            is_circular_sentence("leetcode exercises sound delightful".to_string())
        );
        assert_eq!(true, is_circular_sentence("eetcode".to_string()));
        assert_eq!(false, is_circular_sentence("Leetcode is cool".to_string()));
    }
}

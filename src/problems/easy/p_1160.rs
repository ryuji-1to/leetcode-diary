pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    let mut available_chars = vec![0; 26];
    for c in chars.chars() {
        available_chars[((c as u8) - b'a') as usize] += 1;
    }

    let mut result = 0;
    for word in words {
        let mut tmp = vec![0; 26];
        result += word.len();
        for c in word.chars() {
            let index = ((c as u8) - b'a') as usize;
            tmp[index] += 1;
            available_chars[index];
            if tmp[index] > available_chars[index] {
                result -= word.len();
                break;
            }
        }
    }
    result as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            6,
            count_characters(
                vec![
                    "cat".to_string(),
                    "bt".to_string(),
                    "hat".to_string(),
                    "tree".to_string()
                ],
                "atach".to_string()
            )
        );
        assert_eq!(
            10,
            count_characters(
                vec![
                    "hello".to_string(),
                    "world".to_string(),
                    "leetcode".to_string(),
                ],
                "welldonehoneyr".to_string()
            )
        );
    }
}

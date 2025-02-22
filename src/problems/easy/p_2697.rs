pub fn make_smallrest_palindrome(s: String) -> String {
    let mut result = s.chars().collect::<Vec<_>>();
    let len = result.len();
    for i in 0..(len / 2) {
        let t1 = result[i];
        let t2 = result[len - i - 1];
        // if t1 == t2 {
        //     continue;
        // }
        // if t1 as u8 > t2 as u8 {
        //     result[i] = t2;
        // } else {
        //     result[len - i - 1] = t1;
        // }
        if t1 != t2 {
            let smaller = t1.min(t2);
            result[i] = smaller;
            result[len - i - 1] = smaller;
        }
    }
    result.iter().collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            "efcfe".to_string(),
            make_smallrest_palindrome("egcfe".to_string())
        );
        assert_eq!(
            "abba".to_string(),
            make_smallrest_palindrome("abcd".to_string())
        );
        assert_eq!(
            "neven".to_string(),
            make_smallrest_palindrome("seven".to_string())
        );
    }
}

pub fn make_fancy_string(s: String) -> String {
    let mut prev = ' ';
    let mut count = 1;
    let mut result = "".to_string();
    for c in s.chars() {
        if c == prev {
            count += 1;
        } else {
            prev = c;
            count = 1;
        }

        if count <= 2 {
            result.push(c);
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
            "leetcode".to_string(),
            make_fancy_string("leeetcode".to_string())
        );
        assert_eq!(
            "aabaa".to_string(),
            make_fancy_string("aaabaaaa".to_string())
        );
        assert_eq!("aab".to_string(), make_fancy_string("aab".to_string()));
    }
}

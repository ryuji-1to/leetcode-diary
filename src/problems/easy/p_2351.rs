pub fn repeated_character(s: String) -> char {
    // let mut tmp = Vec::new();
    // for c in s.chars() {
    //     if tmp.contains(&c) {
    //         return c;
    //     }
    //     tmp.push(c);
    // }
    // ' '
    let mut seen = std::collections::HashSet::new();
    s.chars().find(|&c| !seen.insert(c)).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!('c', repeated_character("abccbaacz".to_string()));
        assert_eq!('d', repeated_character("abcdd".to_string()));
    }
}

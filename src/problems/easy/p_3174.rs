pub fn clear_digits(s: String) -> String {
    let mut cs = Vec::new();
    for c in s.chars() {
        match c.to_digit(10) {
            Some(_) => {
                cs.pop();
            }
            None => cs.push(c),
        }
    }
    cs.iter().collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("abc".to_string(), clear_digits("abc".to_string()));
        assert_eq!("".to_string(), clear_digits("cb34".to_string()));
        assert_eq!("a".to_string(), clear_digits("ag3".to_string()));
    }
}

pub fn check_string(s: String) -> bool {
    // !s.chars()
    //     .collect::<Vec<_>>()
    //     .windows(2)
    //     .any(|w| w[0] != 'a' && w[1] == 'a')
    s.matches("ba").count() == 0
    // let mut a = -1;
    // let mut b = -1;
    // for (i, c) in s.chars().enumerate() {
    //     if c == 'a' {
    //         a = i as i32;
    //     } else {
    //         b = i as i32;
    //     }
    //     if b >= 0 && a > b {
    //         return false;
    //     }
    // }
    // true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, check_string("aaabbb".to_string()));
        assert_eq!(false, check_string("abab".to_string()));
        assert_eq!(true, check_string("bbb".to_string()));
    }
}

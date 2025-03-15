pub fn remove_trailing_zeros(num: String) -> String {
    let chars = num.chars().collect::<Vec<char>>();
    let mut i = chars.len() as i32 - 1;

    while i >= 0 {
        if chars[i as usize] == '0' {
            i -= 1;
        } else {
            return chars[0..=i as usize].iter().collect::<String>();
        }
    }
    "".to_string()
    /*
        let sc = num.chars().collect::<Vec<_>>();
        let mut i = sc.len() - 1;
        while sc[i] == '0' {
            i -= 1;
        }
        sc[..=i].iter().collect()
    */
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            "512301".to_string(),
            remove_trailing_zeros("51230100".to_string())
        );
        assert_eq!("123".to_string(), remove_trailing_zeros("123".to_string()));
    }
}

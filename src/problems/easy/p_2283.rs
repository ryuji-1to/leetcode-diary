pub fn digit_count(num: String) -> bool {
    let mut tmp = vec![0; 10];
    for c in num.chars() {
        tmp[c.to_digit(10).unwrap() as usize] += 1;
    }
    num.chars()
        .enumerate()
        .all(|(i, c)| tmp[i] == c.to_digit(10).unwrap())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, digit_count("1210".to_string()));
        assert_eq!(false, digit_count("030".to_string()));
    }
}

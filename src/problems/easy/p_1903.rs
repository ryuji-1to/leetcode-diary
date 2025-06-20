pub fn largest_odd_number(num: String) -> String {
    // let chars = num.chars().collect::<Vec<_>>();
    // for i in 0..chars.len() {
    //     let i = chars.len() - 1 - i;
    //     let target = chars[i].to_digit(10).unwrap();
    //     if target % 2 == 1 {
    //         return chars[..=i].iter().collect::<String>();
    //     }
    // }
    // "".to_string()
    for (i, c) in num.char_indices().rev() {
        if c as u8 & 1 == 1 {
            return num[..=i].to_string();
        }
    }
    String::new()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("5".to_string(), largest_odd_number("52".to_string()));
        assert_eq!("".to_string(), largest_odd_number("4206".to_string()));
        assert_eq!("35427".to_string(), largest_odd_number("35427".to_string()));
    }
}

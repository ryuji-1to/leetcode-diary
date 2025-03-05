/*
pub fn has_same_digits(mut s: String) -> bool {
    while s.len() > 2 {
        let mut next = String::new();
        s = s
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .map(|cs| {
                ((cs[0].to_digit(10).unwrap() + cs[1].to_digit(10).unwrap()) % 10).to_string()
            })
            .collect::<String>();
    }
    &s[0..1] == &s[1..2]
}
*/

pub fn has_same_digits(s: String) -> bool {
    let mut chars: Vec<u32> = s.chars().map(|c| c.to_digit(10).unwrap()).collect();

    while chars.len() > 2 {
        chars = chars.windows(2).map(|cs| (cs[0] + cs[1]) % 10).collect();
    }
    chars[0] == chars[1]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, has_same_digits("3902".to_string()));
        assert_eq!(false, has_same_digits("34789".to_string()));
    }
}

pub fn is_balanced(num: String) -> bool {
    num.chars().enumerate().fold::<i32, _>(0, |acc, (i, v)| {
        if i % 2 == 0 {
            acc + (v as i32 - '0' as i32)
        } else {
            acc - (v as i32 - '0' as i32)
        }
    }) == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(false, is_balanced("1234".to_string()));
        assert_eq!(true, is_balanced("24123".to_string()));
    }
}

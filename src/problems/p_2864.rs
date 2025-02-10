pub fn maximum_odd_binary_number(s: String) -> String {
    let count_s = s.chars().count();
    let count_1 = s.matches("1").count();
    if count_1 == 0 {
        return s;
    }
    format!(
        "{}{}1",
        "1".repeat(count_1 - 1),
        "0".repeat(count_s - count_1)
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            "001".to_string(),
            maximum_odd_binary_number("010".to_string())
        );
        assert_eq!(
            "1001".to_string(),
            maximum_odd_binary_number("0101".to_string())
        );
    }
}

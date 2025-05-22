pub fn reverse_degree(s: String) -> i32 {
    s.chars()
        .enumerate()
        .map(|(i, c)| (b'z' - c as u8 + 1) as i32 * (i + 1) as i32)
        .sum::<i32>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(148, reverse_degree("abc".to_string()));
        assert_eq!(160, reverse_degree("zaza".to_string()));
    }
}

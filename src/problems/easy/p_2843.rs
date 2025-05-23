pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
    (low..=high)
        .map(|n| n.to_string())
        .filter(|s| s.len() % 2 == 0)
        .filter(|s| {
            let mid = s.len() / 2;
            let sum = |part: &str| part.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>();
            sum(&s[..mid]) == sum(&s[mid..])
        })
        .count() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(9, count_symmetric_integers(1, 100));
        assert_eq!(4, count_symmetric_integers(1200, 1230));
    }
}

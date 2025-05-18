pub fn are_numbers_ascending(s: String) -> bool {
    let nums = s
        .split_whitespace()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect::<Vec<_>>();
    nums.windows(2).all(|x| x[1] > x[0])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            true,
            are_numbers_ascending(
                "1 box has 3 blue 4 red 6 green and 12 yellow marbles".to_string()
            )
        );
        assert_eq!(
            false,
            are_numbers_ascending("hello world 5 x 5".to_string())
        );
        assert_eq!(
            false,
            are_numbers_ascending(
                "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s".to_string()
            )
        );
    }
}

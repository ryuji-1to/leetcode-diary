pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
    let v1 = coordinate1.bytes().map(|c| c as i32).sum::<i32>();
    let v2 = coordinate2.bytes().map(|c| c as i32).sum::<i32>();
    (v1 + v2) % 2 == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            true,
            check_two_chessboards("a1".to_string(), "c3".to_string())
        );
        assert_eq!(
            false,
            check_two_chessboards("a1".to_string(), "h3".to_string())
        );
    }
}

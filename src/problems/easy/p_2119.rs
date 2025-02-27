pub fn is_same_after_reversals(num: i32) -> bool {
    num == 0 || num % 10 != 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, is_same_after_reversals(526));
        assert_eq!(false, is_same_after_reversals(1800));
        assert_eq!(true, is_same_after_reversals(0));
    }
}

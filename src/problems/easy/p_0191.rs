pub fn hamming_weight(n: i32) -> i32 {
    n.count_ones() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, hamming_weight(11));
        assert_eq!(1, hamming_weight(128));
        assert_eq!(30, hamming_weight(2147483645));
    }
}

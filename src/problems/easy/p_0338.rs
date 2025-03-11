pub fn count_bits(n: i32) -> Vec<i32> {
    (0..=n).map(|x| x.count_ones() as i32).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![0, 1, 1], count_bits(2));
        assert_eq!(vec![0, 1, 1, 2, 1, 2], count_bits(5));
    }
}

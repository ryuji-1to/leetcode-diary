pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
    (start ^ goal).count_ones() as i32
    // let mut n = start ^ goal;
    // let mut count = 0;
    // while n != 0 {
    //     n = n & (n - 1);
    //     count += 1;
    // }
    // count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // 1010 0111
        assert_eq!(3, min_bit_flips(10, 7));
        // 011 100
        assert_eq!(3, min_bit_flips(3, 4));
    }
}

pub fn hamming_distance(mut x: i32, mut y: i32) -> i32 {
    // let mut result = 0;
    // while x > 0 || y > 0 {
    //     let bit_x = x % 2;
    //     let bit_y = y % 2;
    //     if bit_x != bit_y {
    //         result += 1;
    //     }
    //     x /= 2;
    //     y /= 2;
    // }
    // result
    (x ^ y).count_ones() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, hamming_distance(1, 4));
        assert_eq!(1, hamming_distance(3, 1));
    }
}

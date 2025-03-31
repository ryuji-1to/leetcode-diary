pub fn smallest_number(mut n: i32) -> i32 {
    // let mut count = 0;
    // while n > 0 {
    //     n /= 2;
    //     count += 1;
    // }
    // (0..count).fold(0, |acc, i| acc + 2i32.pow(i))
    let mut x = n;
    let mut r = 0;
    while x != 0 {
        x >>= 1;
        r <<= 1;
        r |= 1;
    }
    r
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(7, smallest_number(5));
        assert_eq!(15, smallest_number(10));
        assert_eq!(3, smallest_number(3));
    }
}

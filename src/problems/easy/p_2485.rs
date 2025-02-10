pub fn pivot_integer(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }
    // let sum1 = (n * (n + 1)) / 2;
    // let mut tmp = 0;
    for i in 1..n {
        if (i * (i + 1)) / 2 == (n * (n + 1)) / 2 - ((i - 1) * i) / 2 {
            return i;
        }
        // tmp += i;
        // if sum1 - tmp + i == tmp {
        //     return i;
        // }
    }
    -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(6, pivot_integer(8));
        assert_eq!(1, pivot_integer(1));
        assert_eq!(-1, pivot_integer(4));
    }
}

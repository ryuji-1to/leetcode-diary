pub fn sum_of_multiples(n: i32) -> i32 {
    // let mut result = 0;
    // for i in 1..=n {
    //     if i % 3 == 0 || i % 5 == 0 || i % 7 == 0 {
    //         result += i;
    //     }
    // }
    // result
    (1..=n).fold(0, |acc, x| {
        if x % 3 == 0 || x % 5 == 0 || x % 7 == 0 {
            acc + x
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(21, sum_of_multiples(7));
        assert_eq!(40, sum_of_multiples(10));
        assert_eq!(30, sum_of_multiples(9));
    }
}

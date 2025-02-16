pub fn can_alice_win(nums: Vec<i32>) -> bool {
    // let (single, double) = nums.iter().fold((0, 0), |mut acc, &x| {
    //     if x >= 10 {
    //         acc.1 += x;
    //     } else {
    //         acc.0 += x
    //     }
    //     acc
    // });
    // single != double;
    nums.iter()
        .map(|&x| if x >= 10 { x } else { -x })
        .sum::<i32>()
        != 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(false, can_alice_win(vec![1, 2, 3, 4, 10]));
        assert_eq!(true, can_alice_win(vec![1, 2, 3, 4, 5, 14]));
        assert_eq!(true, can_alice_win(vec![5, 5, 5, 25]));
    }
}

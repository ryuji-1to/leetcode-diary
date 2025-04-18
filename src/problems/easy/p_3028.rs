pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
    // nums.iter()
    //     .fold((0, 0), |mut acc, &x| {
    //         acc.1 += x;
    //         if acc.1 == 0 {
    //             acc.0 += 1;
    //         }
    //         acc
    //     })
    //     .0
    let mut point = 0;
    let mut result = 0;
    for v in nums {
        point += v;
        if point == 0 {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, return_to_boundary_count(vec![2, 3, -5]));
        assert_eq!(0, return_to_boundary_count(vec![3, 2, -3, -4]));
    }
}

pub fn sum_of_good_numbers(nums: Vec<i32>, k: i32) -> i32 {
    let mut sum = 0;
    let len = nums.len();
    let k = k as usize;
    for i in 0..len {
        let target = nums[i];
        let n1 = if i >= k { nums[i - k] } else { -1 };
        let n2 = if i + k >= len { -1 } else { nums[i + k] };
        if target > n1 && target > n2 {
            sum += target;
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(12, sum_of_good_numbers(vec![1, 3, 2, 1, 5, 4], 2));
        assert_eq!(2, sum_of_good_numbers(vec![2, 1], 1));
    }
}

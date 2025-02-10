pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    // let mut result = 0;
    // for &v in &nums {
    //     if v % 3 != 0 {
    //         result += 1
    //     }
    // }
    // result
    nums.iter()
        .fold(0, |acc, v| if v % 3 != 0 { acc + 1 } else { acc })
}

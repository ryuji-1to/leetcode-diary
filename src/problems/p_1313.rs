pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    // let mut index = 0;
    let mut result = Vec::new();

    // while index * 2 + 1 <= nums.len() {
    //     let freq = nums[2 * index];
    //     let val = nums[2 * index + 1];
    //     for _ in 0..freq {
    //         result.push(val);
    //     }
    //     index += 1;
    // }
    // result

    for i in 0..nums.len() / 2 {
        let freq = nums[2 * i];
        let val = nums[2 * i + 1];
        result.append(&mut [val].repeat(freq as usize));
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![2, 4, 4, 4], decompress_rl_elist(vec![1, 2, 3, 4]));
        assert_eq!(vec![1, 3, 3], decompress_rl_elist(vec![1, 1, 2, 3]));
    }
}

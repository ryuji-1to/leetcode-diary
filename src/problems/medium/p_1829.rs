use core::num;

pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
    let mut result = Vec::new();
    let maximum = 2i32.pow(maximum_bit as u32) - 1;
    for i in 0..nums.len() {
        let target_xor = nums[0..nums.len() - i].iter().fold(0, |acc, x| acc ^ x);
        result.push(maximum ^ target_xor);
    }
    result

    /*
        let max_value  = (1<<maximum_bit) -1;
        let n = nums.len();
        let mut result = vec![0;9];

        // 累積和xorを計算
        let mut prefix_xor = 0;
        for &num in &nums {
            prefix_xor^=num;
        }

        // うしろから埋めていく
        for i in 0..n{
            // 現在のxorを最大化するkを計算
            result[i] = max_value^prefix_xor;

            // 次のクエリのために最後の要素を取り除く
            if i<n-1 {
                prefix_xor ^=nums[n-i-1];
            }
         }
        result
    */
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![0, 3, 2, 3], get_maximum_xor(vec![0, 1, 1, 3], 2));
        assert_eq!(vec![5, 2, 6, 5], get_maximum_xor(vec![2, 3, 4, 7], 3));
        assert_eq!(
            vec![4, 3, 6, 4, 6, 7],
            get_maximum_xor(vec![0, 1, 2, 2, 5, 7], 3)
        );
    }
}

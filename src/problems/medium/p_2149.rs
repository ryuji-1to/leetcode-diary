pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
    let mut positive_nums = Vec::with_capacity(nums.len() / 2);
    let mut negative_nums = Vec::with_capacity(nums.len() / 2);
    let mut result = Vec::with_capacity(nums.len());
    for v in nums {
        if v > 0 {
            positive_nums.push(v);
        } else {
            negative_nums.push(v);
        }
    }
    for i in 0..positive_nums.len() {
        result.push(positive_nums[i]);
        result.push(negative_nums[i]);
    }
    result

    // let mut result = vec![0;nums.len()];
    // let mut positive = 0;
    // let mut negative = 1;
    // for i in nums {
    //     if i < 0 {
    //         result[negative] = i;
    //         negative+=2;
    //     }else {
    //         result[positive]=i;
    //         positive+=2;
    //     }
    // }
    // result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![3, -2, 1, -5, 2, -4],
            rearrange_array(vec![3, 1, -2, -5, 2, -4])
        );
        assert_eq!(vec![1, -1], rearrange_array(vec![-1, 1]));
    }
}

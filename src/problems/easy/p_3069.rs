pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
    let mut arr1 = vec![nums[0]];
    let mut arr2 = vec![nums[1]];
    for i in 2..nums.len() {
        let arr1_last = *arr1.last().unwrap();
        let arr2_last = *arr2.last().unwrap();
        if arr1_last > arr2_last {
            arr1.push(nums[i]);
        } else {
            arr2.push(nums[i]);
        }
    }
    arr1.extend(arr2);
    arr1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![2, 3, 1], result_array(vec![2, 1, 3]));
        assert_eq!(vec![5, 3, 4, 8], result_array(vec![5, 4, 3, 8]));
    }
}

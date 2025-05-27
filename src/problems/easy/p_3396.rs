pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut tmp = vec![];
    for i in 0..len {
        let id = len - i - 1;
        if tmp.contains(&nums[id]) {
            break;
        }
        tmp.push(nums[id]);
    }
    let result = if (len - tmp.len()) % 3 == 0 {
        (len - tmp.len()) / 3
    } else {
        (len - tmp.len()) / 3 + 1
    } as i32;
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, minimum_operations(vec![1, 2, 3, 4, 2, 3, 3, 5, 7]));
        assert_eq!(2, minimum_operations(vec![4, 5, 6, 4, 4]));
        assert_eq!(0, minimum_operations(vec![6, 7, 8, 9]));
    }
}

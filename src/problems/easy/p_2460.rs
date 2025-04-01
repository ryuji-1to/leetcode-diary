pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    for i in 0..n - 1 {
        if nums[i] == nums[i + 1] {
            nums[i] *= 2;
            nums[i + 1] = 0;
        }
    }
    let mut filterd = nums.into_iter().filter(|&x| x > 0).collect::<Vec<i32>>();
    for _ in 0..(n - filterd.len()) {
        filterd.push(0);
    }
    filterd
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![1, 4, 2, 0, 0, 0],
            apply_operations(vec![1, 2, 2, 1, 1, 0])
        );
        assert_eq!(vec![1, 0], apply_operations(vec![0, 1]));
    }
}

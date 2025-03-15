pub fn transform_array(mut nums: Vec<i32>) -> Vec<i32> {
    let mut replaced = nums
        .iter()
        .map(|&x| if x % 2 == 0 { 0 } else { 1 })
        .collect::<Vec<i32>>();
    replaced.sort_unstable();
    replaced
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![0, 0, 1, 1], transform_array(vec![4, 3, 2, 1]));
        assert_eq!(vec![0, 0, 1, 1, 1], transform_array(vec![1, 5, 1, 4, 2]));
    }
}

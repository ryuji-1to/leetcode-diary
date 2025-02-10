pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
    let mut result = Vec::<i32>::new();
    for i in 1..height.len() {
        if height[i - 1] > threshold {
            result.push(i as i32)
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![3, 4], stable_mountains(vec![1, 2, 3, 4, 5], 2));
        assert_eq!(vec![1, 3], stable_mountains(vec![10, 1, 10, 1, 10], 3));
        assert_eq!(
            vec![] as Vec<i32>,
            stable_mountains(vec![10, 1, 10, 1, 10], 10)
        );
    }
}

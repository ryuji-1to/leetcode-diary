pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    for i in 1..mountain.len() - 1 {
        if mountain[i] > mountain[i - 1] && mountain[i] > mountain[i + 1] {
            result.push(i as i32);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![] as Vec<i32>, find_peaks(vec![2, 4, 4]));
        assert_eq!(vec![1, 3], find_peaks(vec![1, 4, 3, 8, 5]));
    }
}

pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
    arr.sort_unstable();
    let diff = arr[1] - arr[0];
    arr.windows(2).all(|x| x[1] - x[0] == diff)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, can_make_arithmetic_progression(vec![3, 5, 1]));
        assert_eq!(false, can_make_arithmetic_progression(vec![1, 2, 4]));
    }
}

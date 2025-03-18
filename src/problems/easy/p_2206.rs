pub fn divide_array(mut nums: Vec<i32>) -> bool {
    nums.sort_unstable();
    nums.chunks(2).all(|x| x[0] == x[1])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, divide_array(vec![3, 2, 3, 2, 2, 2]));
        assert_eq!(false, divide_array(vec![1, 2, 3, 4]));
    }
}

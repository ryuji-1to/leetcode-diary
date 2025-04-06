pub fn count_operations(mut num1: i32, mut num2: i32) -> i32 {
    let mut result = 0;
    while num1 > 0 && num2 > 0 {
        if num1 >= num2 {
            num1 -= num2;
        } else {
            num2 -= num1;
        }
        result += 1;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, count_operations(2, 3));
        assert_eq!(1, count_operations(10, 10));
    }
}

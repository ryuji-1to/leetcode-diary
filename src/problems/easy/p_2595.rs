pub fn even_odd_it(mut n: i32) -> Vec<i32> {
    let mut result = (0, 0);
    let mut index = 0;
    while n > 0 {
        let x = n % 2;
        if x == 1 {
            if index % 2 == 0 {
                result.0 += 1;
            } else {
                result.1 += 1;
            }
        }
        n = n / 2;
        index += 1;
    }
    vec![result.0, result.1]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![1, 2], even_odd_it(50));
        assert_eq!(vec![0, 1], even_odd_it(2));
    }
}

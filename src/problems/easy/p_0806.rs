pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
    let mut result = vec![1, 0];
    for b in s.bytes() {
        let pixel = widths[(b - b'a') as usize];
        if result[1] + pixel > 100 {
            result[0] += 1;
            result[1] = pixel;
        } else {
            result[1] += pixel;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![3, 60],
            number_of_lines(
                vec![
                    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10
                ],
                "abcdefghijklmnopqrstuvwxyz".to_string()
            )
        );
        assert_eq!(
            vec![2, 4],
            number_of_lines(
                vec![
                    4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10
                ],
                "bbbcccdddaaa".to_string()
            )
        );
    }
}

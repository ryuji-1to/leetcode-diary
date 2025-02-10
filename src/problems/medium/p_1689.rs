pub fn min_partitions(n: String) -> i32 {
    // for c in (1..=9).rev() {
    //     if n.contains(&c.to_string()) {
    //         return c;
    //     }
    // }
    // 0
    n.chars().map(|c| c.to_digit(10).unwrap()).max().unwrap() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, min_partitions("32".to_string()));
        assert_eq!(8, min_partitions("82734".to_string()));
        assert_eq!(9, min_partitions("27346209830709182346".to_string()));
    }
}

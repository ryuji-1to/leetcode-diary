pub fn max_containers(n: i32, w: i32, max_weight: i32) -> i32 {
    if n * n * w > max_weight {
        return max_weight / w;
    }
    n * n
    //  (n * n * w).min(max_weight) / w
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, max_containers(2, 3, 15));
        assert_eq!(4, max_containers(3, 5, 20));
    }
}

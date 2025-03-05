pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
    let mut p: Vec<i32> = (1..=m).collect();
    let mut result = Vec::new();
    for v in queries {
        let position = p.iter().position(|&x| x == v).unwrap();
        // p.remove(position);
        // p.insert(0, v);
        result.push(position as i32);
        p[..=position].rotate_right(1);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![2, 1, 2, 1], process_queries(vec![3, 1, 2, 1], 5));
        assert_eq!(vec![3, 1, 2, 0], process_queries(vec![4, 1, 2, 2], 4));
        assert_eq!(vec![6, 5, 0, 7, 5], process_queries(vec![7, 5, 5, 8, 3], 8));
    }
}

pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
    let mut result = vec![first];
    for &v in &encoded {
        result.push(v ^ result[result.len() - 1]);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![1, 0, 2, 1], decode(vec![1, 2, 3], 1));
        assert_eq!(vec![4, 2, 0, 7, 4], decode(vec![6, 2, 7, 3], 4));
    }
}

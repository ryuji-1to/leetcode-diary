pub fn maximum_value(strs: Vec<String>) -> i32 {
    strs.iter()
        .map(|c| match c.parse::<i32>() {
            Ok(v) => v,
            Err(_) => c.len() as i32,
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            5,
            maximum_value(vec![
                "alic3".to_string(),
                "bob".to_string(),
                "3".to_string(),
                "4".to_string(),
                "00000".to_string()
            ])
        );
        assert_eq!(
            1,
            maximum_value(vec![
                "1".to_string(),
                "01".to_string(),
                "001".to_string(),
                "0001".to_string()
            ])
        );
    }
}

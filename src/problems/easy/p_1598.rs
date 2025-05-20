pub fn min_operations(logs: Vec<String>) -> i32 {
    let mut result = 0;
    for v in logs {
        match v.as_str() {
            "../" => result = (result - 1).max(0),
            "./" => continue,
            _ => result += 1,
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
            2,
            min_operations(vec![
                "d1/".to_string(),
                "d2/".to_string(),
                "../".to_string(),
                "d21/".to_string(),
                "./".to_string()
            ])
        );
        assert_eq!(
            3,
            min_operations(vec![
                "d1/".to_string(),
                "d2/".to_string(),
                "./".to_string(),
                "d3/".to_string(),
                "../".to_string(),
                "d31/".to_string(),
            ])
        );
        assert_eq!(
            0,
            min_operations(vec![
                "d1/".to_string(),
                "../".to_string(),
                "../".to_string(),
                "../".to_string(),
            ])
        );
    }
}

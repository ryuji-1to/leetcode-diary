pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
    items.iter().fold(0, |acc, x| match &rule_key[..] {
        "type" => {
            if x[0] == rule_value {
                acc + 1
            } else {
                acc
            }
        }
        "color" => {
            if x[1] == rule_value {
                acc + 1
            } else {
                acc
            }
        }
        "name" => {
            if x[2] == rule_value {
                acc + 1
            } else {
                acc
            }
        }
        _ => acc,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            1,
            count_matches(
                vec![
                    vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
                    vec![
                        "computer".to_string(),
                        "silver".to_string(),
                        "lenovo".to_string()
                    ],
                    vec![
                        "phone".to_string(),
                        "gold".to_string(),
                        "iphone".to_string()
                    ],
                ],
                "color".to_string(),
                "silver".to_string()
            )
        );
        assert_eq!(
            2,
            count_matches(
                vec![
                    vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
                    vec![
                        "computer".to_string(),
                        "silver".to_string(),
                        "phone".to_string()
                    ],
                    vec![
                        "phone".to_string(),
                        "gold".to_string(),
                        "iphone".to_string()
                    ],
                ],
                "type".to_string(),
                "phone".to_string()
            )
        );
    }
}

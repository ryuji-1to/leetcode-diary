pub fn cal_points(operations: Vec<String>) -> i32 {
    let mut points = Vec::<i32>::new();
    for op in operations {
        match op.as_str() {
            "+" => {
                let t1 = points[points.len() - 1];
                let t2 = points[points.len() - 2];
                points.push(t1 + t2);
            }
            "D" => points.push(points.last().unwrap() * 2),
            "C" => {
                points.pop();
            }
            _ => points.push(op.parse::<i32>().unwrap()),
        }
    }
    if points.len() > 0 {
        points.iter().sum()
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            30,
            cal_points(vec![
                "5".to_string(),
                "2".to_string(),
                "C".to_string(),
                "D".to_string(),
                "+".to_string()
            ])
        );
        assert_eq!(
            27,
            cal_points(vec![
                "5".to_string(),
                "-2".to_string(),
                "4".to_string(),
                "C".to_string(),
                "D".to_string(),
                "9".to_string(),
                "+".to_string(),
                "+".to_string()
            ])
        );
        assert_eq!(0, cal_points(vec!["1".to_string(), "C".to_string(),]));
    }
}

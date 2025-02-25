pub fn count_seniors(details: Vec<String>) -> i32 {
    // details.iter().fold(0, |acc, x| {
    //     if x.chars()
    //         .skip(11)
    //         .take(2)
    //         .collect::<String>()
    //         .parse::<i32>()
    //         .unwrap()
    //         > 60
    //     {
    //         acc + 1
    //     } else {
    //         acc
    //     }
    // })
    details
        .iter()
        .filter(|&x| x[11..13].parse::<i32>().unwrap() > 60)
        .count() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            2,
            count_seniors(vec![
                "7868190130M7522".to_string(),
                "5303914400F9211".to_string(),
                "9273338290F4010".to_string()
            ])
        );
        assert_eq!(
            0,
            count_seniors(vec![
                "1313579440F2036".to_string(),
                "2921522980M5644".to_string(),
            ])
        );
    }
}

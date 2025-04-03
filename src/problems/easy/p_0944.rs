pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let mut result = 0;
    let chars = strs
        .iter()
        .map(|xs| xs.chars().collect())
        .collect::<Vec<Vec<char>>>();
    for i in 0..chars[0].len() {
        for j in 0..chars.len() - 1 {
            let current = chars[j][i];
            let next = chars[j + 1][i];
            if current > next {
                result += 1;
                break;
            }
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
            1,
            min_deletion_size(vec![
                "cba".to_string(),
                "daf".to_string(),
                "ghi".to_string()
            ])
        );
        assert_eq!(0, min_deletion_size(vec!["a".to_string(), "b".to_string()]));
        assert_eq!(
            3,
            min_deletion_size(vec![
                "zyx".to_string(),
                "wvu".to_string(),
                "tsr".to_string()
            ])
        );
    }
}

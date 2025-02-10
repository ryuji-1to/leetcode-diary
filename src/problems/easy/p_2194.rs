pub fn cells_in_range(s: String) -> Vec<String> {
    let mut result = Vec::new();
    // let splited = s.split(":").collect::<Vec<_>>();
    // let start = splited[0].chars().collect::<Vec<_>>();
    // let end = splited[1].chars().collect::<Vec<_>>();
    // let list = ('A'..='Z').collect::<Vec<_>>();
    // let start_idx = list.iter().position(|&x| x == start[0]).unwrap();
    // let end_idx = list.iter().position(|&x| x == end[0]).unwrap();
    // for v in &list[start_idx..=end_idx] {
    //     for i in start[1].to_digit(10).unwrap()..=end[1].to_digit(10).unwrap() {
    //         result.push(format!("{}{}", v, i));
    //     }
    // }
    let c1 = s.chars().nth(0).unwrap();
    let r1 = s.chars().nth(1).unwrap();
    let c2 = s.chars().nth(3).unwrap();
    let r2 = s.chars().nth(4).unwrap();
    for c in c1..=c2 {
        for r in r1..=r2 {
            result.push(format!("{}{}", c, r));
        }
    }
    result
}

#[cfg(test)]
mod test {
    use std::cell;

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![
                "K1".to_string(),
                "K2".to_string(),
                "L1".to_string(),
                "L2".to_string()
            ],
            cells_in_range("K1:L2".to_string())
        );
        assert_eq!(
            vec![
                "A1".to_string(),
                "B1".to_string(),
                "C1".to_string(),
                "D1".to_string(),
                "E1".to_string(),
                "F1".to_string()
            ],
            cells_in_range("A1:F1".to_string())
        );
    }
}

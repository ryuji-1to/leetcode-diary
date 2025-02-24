pub fn number_of_beams(bank: Vec<String>) -> i32 {
    let mut result = 0;
    let mut current = 0;
    bank.iter().for_each(|x| {
        if x.contains('1') {
            let count = x.chars().filter(|&x| x == '1').count();
            result += current * count;
            current = count;
        }
    });
    result as i32

    // bank.iter()
    //     .map(|row| row.chars().filter(|&c| c == '1').count() as i32)
    //     .filter(|&count| count > 0)
    //     .scan(0, |current, count| {
    //         let result = *current * count;
    //         *current = count;
    //         Some(result)
    //     })
    //     .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            8,
            number_of_beams(vec![
                "011001".to_string(),
                "000000".to_string(),
                "010100".to_string(),
                "001000".to_string()
            ])
        );
        assert_eq!(
            0,
            number_of_beams(vec![
                "000".to_string(),
                "111".to_string(),
                "000".to_string(),
            ])
        );
    }
}

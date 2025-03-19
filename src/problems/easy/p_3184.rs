pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
    let mut count = 0;
    for i in 0..hours.len() - 1 {
        for j in i + 1..hours.len() {
            if (hours[i] + hours[j]) % 24 == 0 {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, count_complete_day_pairs(vec![12, 12, 30, 24, 24]));
        assert_eq!(3, count_complete_day_pairs(vec![72, 48, 24, 3]));
    }
}

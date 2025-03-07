pub fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
    // let mut current = 0;
    // for v in battery_percentages {
    //     if v > current {
    //         current += 1;
    //     }
    // }
    // current
    battery_percentages
        .iter()
        .fold(0, |acc, &x| if x > acc { acc + 1 } else { acc })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, count_tested_devices(vec![1, 1, 2, 1, 3]));
        assert_eq!(2, count_tested_devices(vec![0, 1, 2]));
    }
}

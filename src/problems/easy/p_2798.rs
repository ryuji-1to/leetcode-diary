pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
    hours
        .iter()
        .fold(0, |acc, &v| if v >= target { acc + 1 } else { acc })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            3,
            number_of_employees_who_met_target(vec![0, 1, 2, 3, 4], 2)
        );
        assert_eq!(
            0,
            number_of_employees_who_met_target(vec![5, 1, 4, 2, 2], 6)
        );
    }
}

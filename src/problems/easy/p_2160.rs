pub fn minimum_sum(num: i32) -> i32 {
    let mut to_num = num
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    to_num.sort_unstable();
    (to_num[0] * 10 + to_num[1] * 10 + to_num[2] + to_num[3]) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(52, minimum_sum(2932));
        assert_eq!(13, minimum_sum(4009));
    }
}

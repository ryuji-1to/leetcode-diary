pub fn split_num(num: i32) -> i32 {
    let mut chars = num
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    chars.sort_unstable_by(|a, b| b.cmp(a));
    let mut result = 0;

    for i in 0..chars.len() {
        let p = (i / 2) as u32;
        result += chars[i] * (10u32.pow(p));
    }
    result as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(59, split_num(4235));
        assert_eq!(75, split_num(687));
        assert_eq!(109998, split_num(999999999));
    }
}

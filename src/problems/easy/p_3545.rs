pub fn min_deletion(s: String, k: i32) -> i32 {
    let mut tmp = vec![0; 26];
    for c in s.chars() {
        tmp[(c as u8 - b'a') as usize] += 1;
    }
    let mut filtered = tmp.into_iter().filter(|&x| x > 0).collect::<Vec<_>>();
    let len = filtered.len() as i32;
    if len <= k {
        return 0;
    }
    filtered.sort_unstable();
    filtered[0..(len - k) as usize].iter().sum::<i32>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, min_deletion("abc".to_string(), 2));
        assert_eq!(0, min_deletion("aabb".to_string(), 2));
        assert_eq!(2, min_deletion("yyyzz".to_string(), 1));
    }
}

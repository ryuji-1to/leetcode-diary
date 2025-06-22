pub fn first_uniq_char(s: String) -> i32 {
    let mut tmp = vec![vec![]; 26];
    for (i, c) in s.chars().enumerate() {
        let index = (c as u8 - b'a') as usize;
        tmp[index].push(i);
    }
    let mut tmp = tmp
        .into_iter()
        .filter(|x| x.len() == 1)
        .map(|x| x[0] as i32)
        .collect::<Vec<_>>();
    tmp.sort_unstable();
    *tmp.get(0).unwrap_or(&-1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(0, first_uniq_char("leetcode".to_string()));
        assert_eq!(2, first_uniq_char("loveleetcode".to_string()));
        assert_eq!(-1, first_uniq_char("aabb".to_string()));
    }
}

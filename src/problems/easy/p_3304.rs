pub fn kth_character(k: i32) -> char {
    let mut result = vec!['a' as u8];
    while result.len() < k as usize {
        let _result = result.iter().map(|&c| c + 1).collect::<Vec<_>>();
        result.extend(_result);
    }
    result[(k - 1) as usize] as char
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!('b', kth_character(5));
        assert_eq!('c', kth_character(10));
    }
}

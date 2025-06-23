pub fn divide_string(mut s: String, k: i32, fill: char) -> Vec<String> {
    // let k = k as usize;
    // let diff = s.len() % k;
    // if diff != 0 {
    //     for _ in 0..(((s.len() / k) + 1) * k - s.len()) {
    //         s.push(fill);
    //     }
    // }

    let k = k as usize;
    // 必要なパディング文字数を計算
    let padding_needed = (k - s.len() % k) % k;

    // パディングを追加
    s.extend(std::iter::repeat(fill).take(padding_needed));
    s.chars()
        .collect::<Vec<_>>()
        .chunks(k as usize)
        .map(|x| x.iter().collect::<String>())
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec!["abc".to_string(), "def".to_string(), "ghi".to_string()],
            divide_string("abcdefghi".to_string(), 3, 'x')
        );
        assert_eq!(
            vec![
                "abc".to_string(),
                "def".to_string(),
                "ghi".to_string(),
                "jxx".to_string()
            ],
            divide_string("abcdefghij".to_string(), 3, 'x')
        );
    }
}

pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
    // let mut result = vec![];
    // let chars = s.chars().collect::<Vec<char>>();
    // let targets = chars
    //     .iter()
    //     .enumerate()
    //     .filter_map(|(i, &_c)| if c == _c { Some(i) } else { None })
    //     .collect::<Vec<usize>>();
    //
    // for i in 0..chars.len() {
    //     let small = targets.iter().map(|&j| (i).abs_diff(j)).min().unwrap();
    //     result.push(small as i32);
    // }
    // result
    let n = s.len();
    let mut result = vec![n as i32; n];
    let chars: Vec<char> = s.chars().collect();

    // 前方からの走査: 左側の最も近い 'c' からの距離を計算
    let mut prev = -(n as i32); // 初期値は十分大きな負の値

    for i in 0..n {
        if chars[i] == c {
            prev = i as i32;
        }
        result[i] = result[i].min(i as i32 - prev);
    }

    // 後方からの走査: 右側の最も近い 'c' からの距離と比較して小さい方を選択
    prev = 2 * (n as i32); // 初期値は十分大きな値

    for i in (0..n).rev() {
        if chars[i] == c {
            prev = i as i32;
        }
        result[i] = result[i].min(prev - i as i32);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0],
            shortest_to_char("loveleetcode".to_string(), 'e')
        );
        assert_eq!(vec![3, 2, 1, 0], shortest_to_char("aaab".to_string(), 'b'));
    }
}

pub fn find_words(words: Vec<String>) -> Vec<String> {
    let mut map = std::collections::HashMap::new();
    let keyboard_layout = vec![
        vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
        vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'],
        vec!['z', 'x', 'c', 'v', 'b', 'n', 'm'],
    ];

    for i in 0..keyboard_layout.len() {
        for &c in &keyboard_layout[i] {
            map.insert(c, i);
        }
    }

    words
        .into_iter()
        .filter(|word| {
            word.chars()
                .map(|c| *map.get(&c.to_ascii_lowercase()).unwrap())
                .collect::<std::collections::HashSet<usize>>()
                .len()
                == 1
        })
        .collect()
    // // キーボードのレイアウトを定義
    //     let keyboard_layout = vec![
    //         vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
    //         vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'],
    //         vec!['z', 'x', 'c', 'v', 'b', 'n', 'm'],
    //     ];
    //
    //     // 各文字の行番号をマッピング
    //     let row_map: std::collections::HashMap<char, usize> = keyboard_layout
    //         .iter()
    //         .enumerate()
    //         .flat_map(|(row, chars)| {
    //             chars.iter().map(move |&c| (c, row))
    //         })
    //         .collect();
    //
    //     // 同じ行のキーのみで構成される単語をフィルタリング
    //     words
    //         .into_iter()
    //         .filter(|word| {
    //             if word.is_empty() {
    //                 return false;
    //             }
    //
    //             let chars = word.chars().collect::<Vec<_>>();
    //             let first_char = chars[0].to_ascii_lowercase();
    //
    //             // 最初の文字の行番号を取得
    //             if let Some(&row) = row_map.get(&first_char) {
    //                 // 全ての文字が同じ行にあるかチェック
    //                 chars[1..].iter()
    //                     .all(|&c| row_map.get(&c.to_ascii_lowercase()).map_or(false, |&r| r == row))
    //             } else {
    //                 false // キーボードに存在しない文字を含む場合
    //             }
    //         })
    //         .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec!["Alaska".to_string(), "Dad".to_string()],
            find_words(vec![
                "Hello".to_string(),
                "Alaska".to_string(),
                "Dad".to_string(),
                "Peace".to_string()
            ])
        );
        assert_eq!(vec![] as Vec<String>, find_words(vec!["omk".to_string(),]));
        assert_eq!(
            vec!["adsdf".to_string(), "sfd".to_string()],
            find_words(vec!["adsdf".to_string(), "sfd".to_string(),])
        );
    }
}

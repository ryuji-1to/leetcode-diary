use std::collections::HashMap;
pub fn num_tile_possibilities(tiles: String) -> i32 {
    // 頻度を計算
    let mut counter = HashMap::<char, i32>::new();
    for c in tiles.chars() {
        *counter.entry(c).or_insert(0) += 1;
    }

    fn backtrack(counter: &mut HashMap<char, i32>) -> i32 {
        let mut total = 0;

        let keys: Vec<char> = counter.keys().cloned().collect();

        for &tile in &keys {
            if counter[&tile] > 0 {
                // 使用する文字を選ぶ
                *counter.get_mut(&tile).unwrap() -= 1;
                // その文字の組み合わせ分の1足して再帰開始
                total += 1 + backtrack(counter);
                // 使用した文字はもとに戻して次のtileへ
                *counter.get_mut(&tile).unwrap() += 1;
            }
        }
        total
    }
    backtrack(&mut counter)
}
/**
 * タイルから作れる文字列の数を計算する関数
 * 
 * 「AAB」の例での計算フロー:
 * 
 * initial call: total = backtrack({A:2, B:1})
 * 
 * backtrack({A:2, B:1}) = 
 *   "A" を選択: 1 + backtrack({A:1, B:1}) +
 *   "B" を選択: 1 + backtrack({A:2, B:0})
 * 
 * backtrack({A:1, B:1}) =
 *   "A" を選択: 1 + backtrack({A:0, B:1}) +
 *   "B" を選択: 1 + backtrack({A:1, B:0})
 * 
 * backtrack({A:0, B:1}) = 
 *   "B" を選択: 1 + backtrack({A:0, B:0}) = 1 + 0 = 1
 * 
 * backtrack({A:1, B:0}) = 
 *   "A" を選択: 1 + backtrack({A:0, B:0}) = 1 + 0 = 1
 * 
 * backtrack({A:2, B:0}) = 
 *   "A" を選択: 1 + backtrack({A:1, B:0}) = 1 + 1 = 2
 * 
 * backtrack({A:0, B:0}) = 0 (使える文字がない)
 * 
 * 組み合わせると:
 * backtrack({A:1, B:1}) = 1 + 1 + 1 + 1 = 4
 * backtrack({A:2, B:0}) = 2
 * thus backtrack({A:2, B:1}) = 1 + 4 + 1 + 2 = 8
 * 
 * 最終結果 total = 8
 * 
 * 作れる文字列:
 * - "A": 1つの文字列
 * - "B": 1つの文字列
 * - "AA": 1つの文字列
 * - "AB": 1つの文字列 
 * - "BA": 1つの文字列
 * - "AAB": 1つの文字列
 * - "ABA": 1つの文字列
 * - "BAA": 1つの文字列
 */

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(8, num_tile_possibilities("AAB".to_string()));
        assert_eq!(188, num_tile_possibilities("AAABBC".to_string()));
        assert_eq!(1, num_tile_possibilities("V".to_string()));
    }
}

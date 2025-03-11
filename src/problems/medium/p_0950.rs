use std::collections::VecDeque;

fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
    // まずデッキをソートする（これが表示される順序）
    deck.sort();
    // 結果を保存するデック（両端キュー）
    let mut result: VecDeque<i32> = VecDeque::new();
    // デッキを逆順に処理
    for &card in deck.iter().rev() {
        // 結果デッキが空でない場合、最後のカードを先頭に移動
        if !result.is_empty() {
            if let Some(last) = result.pop_back() {
                result.push_front(last);
            }
        }
        // 現在のカードをデッキの先頭に追加
        result.push_front(card);
    }
    // VecDequeからVecに変換して返す
    result.into_iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![2, 13, 3, 11, 5, 17, 7],
            deck_revealed_increasing(vec![17, 13, 11, 2, 3, 5, 7])
        );
        assert_eq!(vec![1, 1000], deck_revealed_increasing(vec![1, 1000]));
        assert_eq!(
            vec![1, 4, 2, 6, 3, 5],
            deck_revealed_increasing(vec![1, 2, 3, 4, 5, 6])
        );
    }
}

pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut tmp = [0; 3];
    let char_to_idx = |c: char| -> usize {
        match c {
            'P' => 0,
            'G' => 1,
            'M' => 2,
            _ => panic!("oooo"),
        }
    };

    for i in 0..garbage.len() {
        for c in ['P', 'G', 'M'] {
            if garbage[i].contains(c) {
                let count = garbage[i].chars().filter(|&x| x == c).count();
                result += count as i32;
                let minutes = tmp[char_to_idx(c)];
                result += minutes;
                tmp[char_to_idx(c)] = 0;
            }
            if i != garbage.len() - 1 {
                tmp[char_to_idx(c)] += travel[i];
            }
        }
    }
    result
}

// claude solution
/**
pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
    // 各ゴミの種類について、最後に出現する家のインデックスを記録
    let mut last_house = [-1; 3]; // [M, P, G] の順
    // 各種類のゴミの総量
    let mut total_garbage = [0; 3];

    // 各家のゴミを処理
    for (i, house) in garbage.iter().enumerate() {
        for (j, c) in ['M', 'P', 'G'].iter().enumerate() {
            if house.contains(*c) {
                // このゴミ種類が存在する最後の家を更新
                last_house[j] = i as i32;
                // このゴミの量を加算
                total_garbage[j] += house.chars().filter(|&x| x == *c).count() as i32;
            }
        }
    }

    // 結果を計算
    let mut result = 0;

    // 各ゴミ種類の収集にかかる時間を計算
    for j in 0..3 {
        // このゴミ種類が存在しない場合はスキップ
        if last_house[j] == -1 {
            continue;
        }

        // このゴミの収集にかかる時間
        result += total_garbage[j];

        // このゴミ種類の最後の家までの移動時間
        for i in 0..last_house[j] as usize {
            result += travel[i];
        }
    }

    result
}

// solution2
pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
    // 文字をインデックスにマッピング: M=0, P=1, G=2
    let char_to_idx = |c: char| -> usize {
        match c {
            'M' => 0,
            'P' => 1,
            'G' => 2,
            _ => panic!("不正な文字です"),
        }
    };

    // 各ゴミ種類の総数
    let mut total_counts = [0; 3];
    // 各ゴミ種類が最後に見つかった家
    let mut last_positions = [0; 3];

    // 各家のゴミを処理
    for (i, house) in garbage.iter().enumerate() {
        for c in house.chars() {
            // 各ゴミ種類のカウントを増やす
            total_counts[char_to_idx(c)] += 1;
            // 最後の位置を更新
            last_positions[char_to_idx(c)] = i;
        }
    }

    // 累積移動時間
    let mut prefix_travel = vec![0; travel.len() + 1];
    for i in 0..travel.len() {
        prefix_travel[i+1] = prefix_travel[i] + travel[i];
    }

    // 合計時間を計算
    let mut total_time = 0;

    // 各ゴミ種類の収集時間を計算
    for i in 0..3 {
        // このゴミの収集にかかる時間
        total_time += total_counts[i];

        // このゴミ種類の最後の家までの移動時間
        if last_positions[i] > 0 {
            total_time += prefix_travel[last_positions[i]];
        }
    }

    total_time
}
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            21,
            garbage_collection(
                vec![
                    "G".to_string(),
                    "P".to_string(),
                    "GP".to_string(),
                    "GG".to_string()
                ],
                vec![2, 4, 3]
            )
        );
        assert_eq!(
            37,
            garbage_collection(
                vec!["MMM".to_string(), "PGM".to_string(), "GP".to_string()],
                vec![3, 10]
            )
        );
    }
}

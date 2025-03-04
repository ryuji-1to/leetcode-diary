pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
    // let mut result = vec![vec![r_start, c_start]];
    // let mut base = vec![r_start, c_start];
    // let mut step = 1;
    // let mut current_step = 0;
    // let vector = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    // let mut current_index = 0;
    // let current = vector[current_index];
    //
    // // 1. stepを1でスタート
    // // 2. ->↓<-↑の順で歩く
    // // 3. いったことがあるマスだったら直前の方向にひつつズレて、stepを1アップ
    // // 4. いったことがないマスで範囲ないだったら、resultに追加してcurrent_stepを1あげる
    // // 5. currnet_stepがstepに達したら方向を変えてcurrnet_stepを0にする
    // // 2,5を繰り返す
    // while result.len() < (rows * cols) as usize {
    //     let cordinate = vec![base[0] + current.0, base[1] + current.1];
    //
    //     if result.contains(&cordinate) {
    //         // 行ったことある場合
    //         // 直前の方向にひとつずれる
    //         let prev_index = if current_index == 0 {
    //             vector.len() - 1
    //         } else {
    //             current_index - 1
    //         };
    //         base[0] += vector[prev_index].0;
    //         base[1] += vector[prev_index].1;
    //         // ひとつずれたマスが範囲内だったらresultに追加する
    //         if in_range(&base, rows - 1, cols - 1) {
    //             result.push(base.clone());
    //         }
    //         step += 1;
    //     } else {
    //         // 行ったことない場合
    //         if in_range(&cordinate, rows - 1, cols - 1) {
    //             result.push(cordinate.clone());
    //         }
    //         current_step += 1;
    //         base[0] = cordinate[0];
    //         base[1] = cordinate[1];
    //     }
    //
    //     if current_step == step {
    //         current_step = 0;
    //         if current_index == vector.len() - 1 {
    //             current_index = 0;
    //         } else {
    //             current_index += 1;
    //         };
    //     }
    // }

    // 方針: 偶数回ごとにステップ数を増やしスパイラルを表現する
    let mut result = Vec::with_capacity((rows * cols) as usize);
    result.push(vec![r_start, c_start]);
    // 4方向のベクトルを定義（-> ↓ <- ↑）
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    // 現在の位置
    let mut r = r_start;
    let mut c = c_start;
    // 移動方向のインデックス
    let mut dir_index = 0;
    // ステップ数
    let mut step_length = 0;

    while result.len() < (rows * cols) as usize {
        // 偶数回目の方向転換で、stepを1増やす
        if dir_index % 2 == 0 {
            step_length += 1;
        }
        // 現在の方向で指定されたステップ数移動
        for _ in 0..step_length {
            // 次の座標に移動
            r += directions[dir_index].0;
            c += directions[dir_index].1;

            // グリッド内の座標のみ結果に追加
            if r >= 0 && r < rows && c >= 0 && c < cols {
                result.push(vec![r, c]);
            }
        }
        // 方向を時計回りに変更
        dir_index = (dir_index + 1) % 4;
    }
    result
}

// fn in_range(cordicate: &Vec<i32>, x: i32, y: i32) -> bool {
//     (cordicate[0] > 0 && cordicate[1] > 0) && (cordicate[0] > x || cordicate[1] > y)
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let expected1 = vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3]];
        assert_eq!(expected1, spiral_matrix_iii(1, 4, 0, 0));
        let expected2 = vec![
            vec![1, 4],
            vec![1, 5],
            vec![2, 5],
            vec![2, 4],
            vec![2, 3],
            vec![1, 3],
            vec![0, 3],
            vec![0, 4],
            vec![0, 5],
            vec![3, 5],
            vec![3, 4],
            vec![3, 3],
            vec![3, 2],
            vec![2, 2],
            vec![1, 2],
            vec![0, 2],
            vec![4, 5],
            vec![4, 4],
            vec![4, 3],
            vec![4, 2],
            vec![4, 1],
            vec![3, 1],
            vec![2, 1],
            vec![1, 1],
            vec![0, 1],
            vec![4, 0],
            vec![3, 0],
            vec![2, 0],
            vec![1, 0],
            vec![0, 0],
        ];
        assert_eq!(expected2, spiral_matrix_iii(5, 6, 1, 4));
    }
}

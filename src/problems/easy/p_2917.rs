pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
    // let mut tmp = vec![0; 32];
    // for mut v in nums {
    //     for i in 0..32 {
    //         if (v == 0) {
    //             break;
    //         }
    //         tmp[i] += v % 2;
    //         v /= 2;
    //     }
    // }
    // tmp.iter()
    //     .map(|&x| if x >= k { 1 } else { 0 })
    //     .enumerate()
    //     .map(|(i, v)| if v == 1 { 2i32.pow(i as u32) } else { 0 })
    //     .sum::<i32>()
    // 各ビット位置でのカウントを保持する配列
    let mut bit_counts = [0; 32];

    // 各数値のビットを調べる
    for num in nums {
        // 32ビット分ループ
        for bit_pos in 0..32 {
            // i番目のビットが1かどうかをチェック
            if (num & (1 << bit_pos)) != 0 {
                bit_counts[bit_pos] += 1;
            }
        }
    }

    // 結果を計算
    let mut result = 0;
    for bit_pos in 0..32 {
        // そのビット位置で少なくともk個の数が1を持っているか
        if bit_counts[bit_pos] >= k {
            result |= 1 << bit_pos;
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(9, find_k_or(vec![7, 12, 9, 8, 9, 15], 4));
        assert_eq!(0, find_k_or(vec![2, 12, 1, 11, 4, 5], 6));
        assert_eq!(15, find_k_or(vec![10, 8, 5, 9, 11, 6, 8], 1));
    }
}

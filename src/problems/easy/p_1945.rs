pub fn get_lucky(s: String, k: i32) -> i32 {
    let mut s_to_nums = s
        .bytes()
        .map(|c| ((c - b'a') + 1).to_string())
        .collect::<String>();
    for i in 0..k - 1 {
        s_to_nums = s_to_nums
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i32)
            .sum::<i32>()
            .to_string();
    }
    s_to_nums
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .sum::<i32>()
}
/*
pub fn get_lucky(s: String, k: i32) -> i32 {
    // 最初の変換を実行（文字をアルファベット位置に変換して合計する）
    let mut sum = s.bytes().map(|b| (b - b'a' + 1) as i32).sum::<i32>();
    
    // k-1回の繰り返し計算（各桁の合計を計算）
    for _ in 0..k-1 {
        sum = digit_sum(sum);
    }
    
    sum
}

// 数字の各桁の合計を計算する補助関数
fn digit_sum(mut n: i32) -> i32 {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(36, get_lucky("iiii".to_string(), 1));
        assert_eq!(6, get_lucky("leetcode".to_string(), 2));
        assert_eq!(8, get_lucky("zbax".to_string(), 2));
    }
}

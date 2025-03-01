pub fn count_triplets(arr: Vec<i32>) -> i32 {
    /*
    let mut result = 0;
    for i in 0..arr.len() - 1 {
        for j in i + 1..arr.len() {
            // i ~ j
            let left = arr[i..j].iter().fold(0, |acc, x| acc ^ x);
            // j ~ k
            let _ = arr[j..].iter().fold(0, |acc, x| {
                let r = acc ^ x;
                if left == r {
                    result += 1;
                }
                r
            });
        }
    }
    result
     */

    /*
        let n = arr.len();
        let mut count = 0;
        // prefix
        let mut prefix = vec![0; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] ^ arr[i];
        }
        for i in 0..n {
            for k in i + 1..n {
                for j in i + 1..=k {
                    let a = prefix[j] ^ prefix[i];
                    let b = prefix[k + 1] ^ prefix[j];
                    if a == b {
                        count += 1
                    }
                }
            }
        }
        count
    */

    /**
    * XORトリプレットを数える関数
    * 
    * 3つのインデックス i, j, k を選び、以下の条件を満たすトリプレットの数を返す：
    * - 0 <= i < j <= k < arr.length
    * - a = arr[i] ^ arr[i+1] ^ ... ^ arr[j-1]
    * - b = arr[j] ^ arr[j+1] ^ ... ^ arr[k]
    * - a == b
    * 
    * この問題は、累積XOR（プレフィックスXOR）を使用して効率的に解ける。
    * XORの特性により、a == b の条件は prefix[i] == prefix[k+1] と等価になる。
    */
    let n = arr.len();
    let mut count = 0;

    let mut prefix = vec![0; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] ^ arr[i];
    }

    for i in 0..n {
        for k in i + 1..n {
            if prefix[i] == prefix[k + 1] {
                count += k - i;
            }
        }
    }
    count as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, count_triplets(vec![2, 3, 1, 6, 7]));
        assert_eq!(10, count_triplets(vec![1, 1, 1, 1, 1]));
    }
}

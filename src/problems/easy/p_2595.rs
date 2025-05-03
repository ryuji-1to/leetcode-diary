pub fn even_odd_it(mut n: i32) -> Vec<i32> {
    // let mut result = (0, 0);
    // let mut index = 0;
    // while n > 0 {
    //     let x = n % 2;
    //     if x == 1 {
    //         if index % 2 == 0 {
    //             result.0 += 1;
    //         } else {
    //             result.1 += 1;
    //         }
    //     }
    //     n = n / 2;
    //     index += 1;
    // }
    // vec![result.0, result.1]

    // 偶数ビットと奇数ビットをマスクで分離
    let even_bits = n & 0x5555; // 01010101...のマスク（偶数位置のビット）
    let odd_bits = n & 0xAAAA; // 10101010...のマスク（奇数位置のビット）

    // それぞれのビット数をカウント
    let even_count = even_bits.count_ones() as i32;
    let odd_count = odd_bits.count_ones() as i32;

    vec![even_count, odd_count]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![1, 2], even_odd_it(50));
        assert_eq!(vec![0, 1], even_odd_it(2));
    }
}

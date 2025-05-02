// pub fn distribute_candies(n: i32, limit: i32) -> i32 {
//     let mut count = 0;
//     for i in 0..=limit {
//         for j in 0..=limit {
//             for k in 0..=limit {
//                 if i + j + k == n {
//                     count += 1;
//                 }
//             }
//         }
//     }
//     count
// }

pub fn distribute_candies(n: i32, limit: i32) -> i32 {
    let mut total = comb(n + 2, 2);

    for mask in 1..8 {
        let bits = (mask as u32).count_ones();
        let over = bits * (limit + 1) as u32;
        if n as u32 >= over {
            let remain = n - (over as i32);
            let sign = if bits % 2 == 1 { -1 } else { 1 };
            total += sign * comb(remain + 2, 2);
        }
    }

    total
}

fn comb(n: i32, k: i32) -> i32 {
    if n < k || k < 0 {
        return 0;
    }
    (1..=k).fold(1, |acc, i| acc * (n - i + 1) / i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, distribute_candies(5, 2));
        assert_eq!(10, distribute_candies(3, 3));
    }
}

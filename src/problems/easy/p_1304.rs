pub fn sum_zero(n: i32) -> Vec<i32> {
    let mut plus = (1..n).collect::<Vec<i32>>();
    plus.push(plus.iter().sum::<i32>() * -1);
    plus
    // (1..n).chain(std::iter::once(-n * (n - 1) / 2)).collect()
}

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         assert_eq!(vec![-7, -1, 1, 3, 4], sum_zero(5));
//         assert_eq!(vec![-1, 0, 1], sum_zero(3));
//         assert_eq!(vec![0], sum_zero(1));
//     }
// }

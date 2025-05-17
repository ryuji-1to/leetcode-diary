pub fn min_cost_of_to_move_chips(position: Vec<i32>) -> i32 {
    // let mut even_sum = 0;
    // let mut odd_sum = 0;
    // for v in position {
    //     if v % 2 == 0 {
    //         even_sum += 1;
    //     } else {
    //         odd_sum += 1;
    //     }
    // }
    // even_sum.min(odd_sum)

    let even_count = position.iter().filter(|&&x| x % 2 == 0).count() as i32;
    even_count.min(position.len() as i32 - even_count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, min_cost_of_to_move_chips(vec![1, 2, 3]));
        assert_eq!(2, min_cost_of_to_move_chips(vec![2, 2, 2, 3, 3]));
        assert_eq!(1, min_cost_of_to_move_chips(vec![1, 1000000000]));
    }
}

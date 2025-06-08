pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
    let can_eat_count = candy_type.len() / 2;
    // let candies = candy_type
    //     .into_iter()
    //     .collect::<std::collections::HashSet<_>>();
    let mut candies = std::collections::HashSet::<i32>::new();
    for c in candy_type {
        candies.insert(c);
        if candies.len() == can_eat_count {
            return can_eat_count as i32;
        }
    }
    candies.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, distribute_candies(vec![1, 1, 2, 2, 3, 3]));
        assert_eq!(2, distribute_candies(vec![1, 1, 2, 3]));
        assert_eq!(1, distribute_candies(vec![6, 6, 6, 6]));
    }
}

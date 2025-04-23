pub fn min_costs(cost: Vec<i32>) -> Vec<i32> {
    // let mut result = vec![];
    // for i in 0..cost.len() {
    //     let target = &cost[0..=i];
    //     let min_cost = *target.iter().min().unwrap();
    //     result.push(min_cost);
    // }
    // result
    cost.iter()
        .enumerate()
        .map(|(i, _)| *cost[0..=i].iter().min().unwrap())
        .collect()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![5, 3, 3, 1, 1, 1], min_costs(vec![5, 3, 4, 1, 3, 2]));
        assert_eq!(vec![1, 1, 1, 1, 1], min_costs(vec![1, 2, 4, 6, 7]));
    }
}

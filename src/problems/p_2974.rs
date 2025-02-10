pub fn number_game(mut nums: Vec<i32>) -> Vec<i32> {
    // let mut result = Vec::new();
    nums.sort_unstable();
    // nums.chunks(2).for_each(|x| {
    //     result.push(x[1]);
    //     result.push(x[0]);
    // });
    // result
    nums.chunks(2)
        .flat_map(|x| vec![x[1], x[0]])
        .collect::<Vec<i32>>()
    // while nums.len() > 0 {
    //     let a = *nums.iter().min().unwrap();
    //     nums.remove(*nums.iter().find(|&&x| x == a).unwrap() as usize);
    //     let b = *nums.iter().min().unwrap();
    //     nums.remove(*nums.iter().find(|&&x| x == b).unwrap() as usize);
    //     result.push(a);
    //     result.push(b);
    // }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![3, 2, 5, 4], number_game(vec![5, 4, 2, 3]));
        assert_eq!(vec![5, 2], number_game(vec![2, 5]));
    }
}

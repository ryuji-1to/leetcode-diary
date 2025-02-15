pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    // let mut tmp_a = Vec::new();
    // let mut tmp_b = Vec::new();
    // let mut result = Vec::new();
    // a.iter().zip(b).for_each(|(&a, b)| {
    //     tmp_a.push(a);
    //     tmp_b.push(b);
    //     let common_count = tmp_a
    //         .iter()
    //         .fold(0, |acc, x| if tmp_b.contains(x) { acc + 1 } else { acc });
    //     result.push(common_count);
    // });
    // result
    a.into_iter()
        .zip(b)
        .scan([0_usize; 2], |[ao, bo], (a, b)| {
            *ao |= 1 << a;
            *bo |= 1 << b;
            Some((*ao & *bo).count_ones() as i32)
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![0, 2, 3, 4],
            find_the_prefix_common_array(vec![1, 3, 2, 4], vec![3, 1, 2, 4])
        );
        assert_eq!(
            vec![0, 1, 3],
            find_the_prefix_common_array(vec![2, 3, 1], vec![3, 1, 2])
        );
    }
}

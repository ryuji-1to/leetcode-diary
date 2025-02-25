pub fn pivot_array(mut nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut pivots = Vec::new();

    for v in nums {
        if v == pivot {
            pivots.push(v);
        } else if v < pivot {
            left.push(v);
        } else {
            right.push(v);
        }
    }
    left.extend(pivots);
    left.extend(right);
    left
    // nums.sort_by_key(|num| num.cmp(&pivot));
    // nums
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![9, 5, 3, 10, 10, 12, 14],
            pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10)
        );
        assert_eq!(vec![-3, 2, 4, 3], pivot_array(vec![-3, 4, 3, 2], 2));
    }
}

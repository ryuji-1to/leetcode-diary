pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
    let mut s = 0;
    let mut s2 = 0;

    for v in nums {
        s += v;
        if v < 10 {
            s2 += v;
            continue;
        }
        let mut v2 = v;
        while v2 != 0 {
            s2 += v2 % 10;
            v2 /= 10;
        }
    }
    (s - s2).abs()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(9, difference_of_sum(vec![1, 15, 6, 3]));
        assert_eq!(0, difference_of_sum(vec![1, 2, 3, 4]));
    }
}

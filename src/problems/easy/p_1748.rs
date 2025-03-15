use std::collections::HashMap;

pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
    // let mut result = 0;
    // let mut map = HashMap::new();
    // for v in nums {
    //     *map.entry(v).or_insert(0) += 1;
    // }
    // for (k, v) in map {
    //     if v == 1 {
    //         result += k;
    //     }
    // }
    // result
    let mut tmp = vec![0; 100];
    let mut result = 0;
    for v in nums {
        let id = (v - 1) as usize;
        if tmp[id] == 0 {
            result += v;
        } else if tmp[id] == 1 {
            result -= v;
        }
        tmp[id] += 1;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, sum_of_unique(vec![1, 2, 3, 2]));
        assert_eq!(0, sum_of_unique(vec![1, 1, 1, 1, 1]));
        assert_eq!(15, sum_of_unique(vec![1, 2, 3, 4, 5]));
    }
}

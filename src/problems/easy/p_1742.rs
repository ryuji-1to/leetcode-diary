use std::collections::HashMap;

pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
    let mut map = HashMap::<usize, i32>::new();
    let mut result = 0;
    for i in (low_limit..=high_limit) {
        let mut num = i;
        let mut sum = 0;
        while num > 0 {
            sum += num % 10;
            num /= 10;
        }
        *map.entry(sum as usize).or_insert(0) += 1;
    }
    *map.values().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, count_balls(1, 10));
        assert_eq!(2, count_balls(5, 15));
        assert_eq!(2, count_balls(19, 28));
    }
}

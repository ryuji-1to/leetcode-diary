pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    // let mut result = vec![0];
    // for v in gain {
    //     result.push(result.last().unwrap() + v);
    // }
    // *result.iter().max().unwrap()
    gain.iter()
        .fold((0, 0), |(c, m), &x| (c + x, m.max(c + x)))
        .1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, largest_altitude(vec![-5, 1, 5, 0, -7]));
        assert_eq!(0, largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]));
    }
}

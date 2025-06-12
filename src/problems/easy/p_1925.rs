pub fn count_triples(n: i32) -> i32 {
    let mut result = 0;
    for a in 1..n {
        for b in a..n {
            let sum = a.pow(2) + b.pow(2);
            let sqrt = (sum as f64).sqrt();
            if sqrt.fract() == 0.0 && sqrt as i32 <= n {
                result += 1;
            }
        }
    }
    result * 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, count_triples(5));
        assert_eq!(4, count_triples(10));
    }
}

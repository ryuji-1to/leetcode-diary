pub fn generate_key(mut num1: i32, mut num2: i32, mut num3: i32) -> i32 {
    let mut result = 0;
    for i in 0..4 {
        let n1 = num1 % 10;
        let n2 = num2 % 10;
        let n3 = num3 % 10;
        let suffix = if i > 0 { (10i32).pow(i) } else { 1 };
        result += (n1.min(n2).min(n3)) * suffix;
        num1 /= 10;
        num2 /= 10;
        num3 /= 10;
    }
    result

    /*
    std::iter::successors(Some((num1, num2, num3)), |&(n1, n2, n3)| {
        if n1 == 0 && n2 == 0 && n3 == 0 {
            None
        } else {
            Some((n1 / 10, n2 / 10, n3 / 10))
        }
    })
    .take(4)
    .enumerate()
    .fold(0, |acc, (i, (n1, n2, n3))| {
        acc + (n1 % 10).min(n2 % 10).min(n3 % 10) * 10i32.pow(i as u32)
    })
    */
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(0, generate_key(1, 10, 1000));
        assert_eq!(777, generate_key(987, 879, 798));
        assert_eq!(1, generate_key(1, 2, 3));
    }
}

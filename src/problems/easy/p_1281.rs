pub fn subtract_product_and_sum(mut n: i32) -> i32 {
    // let (p, s) = n
    //     .to_string()
    //     .chars()
    //     .fold((1, 0), |acc, x| match x.to_digit(10) {
    //         Some(v) => (acc.0 * v, acc.1 + v),
    //         None => acc,
    //     });
    // (p - s) as i32
    let mut sum: i32 = n % 10;
    let mut product: i32 = n % 10;
    while n / 10 != 0 {
        n /= 10;
        sum += n % 10;
        product *= n % 10;
    }
    product - sum
}

#[cfg(test)]
mod tests {
    use super::*;

    fn it_works() {
        assert_eq!(15, subtract_product_and_sum(234));
        assert_eq!(21, subtract_product_and_sum(4421));
    }
}

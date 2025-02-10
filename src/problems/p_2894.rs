pub fn difference_of_sums(n: i32, m: i32) -> i32 {
    // let arr: Vec<i32> = (1..=n).collect();
    // let num1: i32 = arr.iter().filter(|&x| x % m != 0).sum();
    // let num2: i32 = arr.iter().filter(|&x| x % m == 0).sum();
    // num1 - num2
    (1..=n).map(|x| if x % m == 0 { -x } else { x }).sum()
}

pub fn time_required_to_buy(mut tickets: Vec<i32>, k: i32) -> i32 {
    // let mut result = 0;
    // let mut index = 0;
    // while tickets[k as usize] > 0 {
    //     if tickets[index] > 0 {
    //         tickets[index] -= 1;
    //         result += 1;
    //     }
    //     if index == tickets.len() - 1 {
    //         index = 0;
    //     } else {
    //         index += 1;
    //     }
    // }
    // result
    let k = k as usize;
    let target = tickets[k];
    let mut time = 0;
    for (i, &ticket) in tickets.iter().enumerate() {
        if i <= k {
            time += ticket.min(target);
        } else {
            time += ticket.min(target - 1);
        }
    }
    time
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(6, time_required_to_buy(vec![2, 3, 2], 2));
        assert_eq!(8, time_required_to_buy(vec![5, 1, 1, 1], 0));
    }
}

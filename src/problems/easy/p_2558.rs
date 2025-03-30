use std::collections::BinaryHeap;

pub fn pick_gifts(mut gifts: Vec<i32>, k: i32) -> i64 {
    // gifts.iter().sum::<i32>() as i64
    // for _ in 0..k {
    //     let max = *gifts.iter().max().unwrap();
    //     let position = gifts.iter().position(|&x| x == max).unwrap();
    //     gifts[position] = (max as f64).sqrt().floor() as i32;
    // }
    // gifts.iter().map(|&x| x as i64).sum()
    let mut heap = BinaryHeap::from(gifts);
    for _ in 0..k {
        if let Some(max) = heap.pop() {
            heap.push((max as f64).sqrt().floor() as i32);
        }
    }
    heap.iter().map(|&x| x as i64).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(29, pick_gifts(vec![25, 64, 9, 4, 100], 4));
        assert_eq!(4, pick_gifts(vec![1, 1, 1, 1,], 4));
    }
}

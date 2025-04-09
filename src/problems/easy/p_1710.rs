pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
    // box_types.sort_unstable_by(|a, b| b[1].cmp(&a[1]));
    // box_types.iter().fold(0, |acc, x| {
    //     if truck_size <= 0 {
    //         return acc;
    //     }
    //     let tmp = if x[0] > truck_size {
    //         truck_size * x[1]
    //     } else {
    //         x[0] * x[1]
    //     };
    //     truck_size -= x[0];
    //     acc + tmp
    // })
    box_types.sort_unstable_by_key(|b| -b[1]);
    let mut total_units = 0;

    for box_type in box_types {
        if truck_size <= 0 {
            break;
        }

        let boxes_to_take = box_type[0].min(truck_size);
        total_units += boxes_to_take * box_type[1];
        truck_size -= boxes_to_take;
    }

    total_units
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            8,
            maximum_units(vec![vec![1, 3], vec![2, 2], vec![3, 1]], 4)
        );
        assert_eq!(
            91,
            maximum_units(vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]], 10)
        );
    }
}

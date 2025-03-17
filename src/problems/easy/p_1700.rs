pub fn count_students(mut students: Vec<i32>, mut sandwiches: Vec<i32>) -> i32 {
    while students.contains(sandwiches.get(0).unwrap_or(&-1)) {
        if sandwiches[0] == students[0] {
            sandwiches.remove(0);
        } else {
            students.push(students[0]);
        }
        students.remove(0);
    }
    students.len() as i32
    // loop {
    //     if let Some(sandwich) = sandwiches.get(0) {
    //         if !students.contains(sandwich) {
    //             return students.len() as i32;
    //         }
    //         if *sandwich == students[0] {
    //             sandwiches.remove(0);
    //         } else {
    //             students.push(students[0]);
    //         }
    //         students.remove(0);
    //     } else {
    //         return 0;
    //     }
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(0, count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]));
        assert_eq!(
            3,
            count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1])
        );
    }
}

pub fn flip_and_invert_image(mut image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for i in 0..image.len() {
        let row_len = image[i].len();
        for j in 0..row_len {
            if image[i][j] == 1 {
                image[i][j] = 0;
            } else {
                image[i][j] = 1;
            }
        }
        image[i].reverse();
    }
    image
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]],
            flip_and_invert_image(vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]])
        );
        assert_eq!(
            vec![
                vec![1, 1, 0, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 1],
                vec![1, 0, 1, 0]
            ],
            flip_and_invert_image(vec![
                vec![1, 1, 0, 0],
                vec![1, 0, 0, 1],
                vec![0, 1, 1, 1],
                vec![1, 0, 1, 0]
            ])
        );
    }
}

pub fn count_points(rings: String) -> i32 {
    // let mut tmp: Vec<Vec<char>> = vec![vec![]; 10];
    // rings
    //     .chars()
    //     .collect::<Vec<_>>()
    //     .chunks(2)
    //     .for_each(|c| tmp[c[1].to_digit(10).unwrap() as usize].push(c[0]));
    //
    // tmp.iter()
    //     .filter(|&v| v.contains(&'R') && v.contains(&'G') && v.contains(&'B'))
    //     .count() as i32
    //

    /**
     solution by claude

     let mut rods: Vec<Vec<char>> = vec![vec![]; 10];

     // 2文字ずつ処理
     let chars: Vec<char> = rings.chars().collect();
     for i in (0..chars.len()).step_by(2) {
         let color = chars[i];
         let rod = chars[i+1].to_digit(10).unwrap() as usize;
         rods[rod].push(color);
     }

     // 3色すべてを持つロッドをカウント
     rods.iter()
         .filter(|&colors| colors.contains(&'R') && colors.contains(&'G') && colors.contains(&'B'))
         .count() as i32
    */
    let mut rod_colors = [0u8; 10];

    let chars: Vec<char> = rings.chars().collect();
    for i in (0..chars.len()).step_by(2) {
        let color = match chars[i] {
            'R' => 1,
            'G' => 2,
            'B' => 4,
            _ => 0,
        };
        let rod = chars[i + 1].to_digit(10).unwrap() as usize;
        rod_colors[rod] |= color;
    }
    rod_colors.iter().filter(|&&color| color == 7).count() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, count_points("B0B6G0R6R0R6G9".to_string()));
        assert_eq!(1, count_points("B0R0G0R9R0B0G0".to_string()));
        assert_eq!(0, count_points("G4".to_string()));
    }
}

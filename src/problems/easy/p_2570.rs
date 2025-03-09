pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut i = 0;
    let mut j = 0;
    let mut k = 1;
    let mut result = Vec::new();

    while i != nums1.len() || j != nums2.len() {
        let got1 = nums1.get(i);
        let got2 = nums2.get(j);
        let mut tmp = vec![k as i32, 0];
        if let Some(v) = got1 {
            if (v[0] == k) {
                tmp[1] += v[1];
                i += 1;
            }
        }
        if let Some(v) = got2 {
            if (v[0] == k) {
                tmp[1] += v[1];
                j += 1;
            }
        }
        if tmp[1] > 0 {
            result.push(tmp);
        }
        k += 1;
    }
    result
}

/*
pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  let mut result = Vec::new();
  let mut i = 0;
  let mut j = 0;
  
  // 両方の配列が終わるまでマージ
  while i < nums1.len() || j < nums2.len() {
      if i < nums1.len() && (j == nums2.len() || nums1[i][0] < nums2[j][0]) {
          // nums1の要素だけが存在または小さい場合
          result.push(nums1[i].clone());
          i += 1;
      } else if j < nums2.len() && (i == nums1.len() || nums1[i][0] > nums2[j][0]) {
          // nums2の要素だけが存在または小さい場合
          result.push(nums2[j].clone());
          j += 1;
      } else {
          // 両方の配列に同じIDがある場合
          let id = nums1[i][0];
          let sum_val = nums1[i][1] + nums2[j][1];
          result.push(vec![id, sum_val]);
          i += 1;
          j += 1;
      }
  }
    
  result
}
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![vec![1, 6], vec![2, 3], vec![3, 2], vec![4, 6]],
            merge_arrays(
                vec![vec![1, 2], vec![2, 3], vec![4, 5]],
                vec![vec![1, 4], vec![3, 2], vec![4, 1]]
            )
        );
        assert_eq!(
            vec![vec![1, 3], vec![2, 4], vec![3, 6], vec![4, 3], vec![5, 5]],
            merge_arrays(
                vec![vec![2, 4], vec![3, 6], vec![5, 5]],
                vec![vec![1, 3], vec![4, 3]]
            )
        );
    }
}

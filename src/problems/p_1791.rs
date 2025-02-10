struct Solution;

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        // let first = edges.first().unwrap();
        // let a1 = first[0];
        // let a2 = first[1];
        // for v in edges {
        //     if !v.contains(&a1) {
        //         return a2;
        //     }
        //     if !v.contains(&a2) {
        //         return a1;
        //     }
        // }
        // 0
        let first = &edges[0];
        let second = &edges[1];
        if first[0] == second[0] || first[0] == second[1] {
            first[0]
        } else {
            first[1]
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    // if root.is_none() {
    //     return 0;
    // }
    // let node = root.unwrap();
    // let node_ref = node.borrow();
    // let val = node_ref.val;
    //
    // if val < low {
    //     return range_sum_bst(node_ref.right.clone(), low, high);
    // } else if val > high {
    //     return range_sum_bst(node_ref.left.clone(), low, high);
    // } else {
    //     return val
    //         + range_sum_bst(node_ref.left.clone(), low, high)
    //         + range_sum_bst(node_ref.right.clone(), low, high);
    // }
    // RC -> シングルスレッドで所有権を共有するもの
    // RefCellはランタイムに借用規則を移すもの。参照と可変参照を安全にするもの
    if root.is_none() {
        return 0;
    }
    let node = root.unwrap();
    let node_ref = node.borrow();
    let val = node_ref.val;
    if val < low {
        return range_sum_bst(node_ref.right.clone(), low, high);
    } else if val > high {
        return range_sum_bst(node_ref.left.clone(), low, high);
    } else {
        return val
            + range_sum_bst(node_ref.right.clone(), low, high)
            + range_sum_bst(node_ref.left.clone(), low, high);
    }
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         assert_eq!(32, range_sum_bst(, 7, 15));
//     }
// }

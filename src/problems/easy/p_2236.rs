use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if root.is_none() {
        return false;
    }
    // let root = root.unwrap();
    // let root_ref = root.borrow();
    // let left_num = root_ref.left.clone().unwrap().borrow().val;
    // let right_num = root_ref.right.clone().unwrap().borrow().val;
    // root_ref.val == left_num + right_num
    let root_node = root.as_ref().unwrap().borrow();
    let root_val = root_node.val;
    let left_val = root_node.left.as_ref().unwrap().borrow().val;
    let right_val = root_node.right.as_ref().unwrap().borrow().val;
    root_val == left_val + right_val
}

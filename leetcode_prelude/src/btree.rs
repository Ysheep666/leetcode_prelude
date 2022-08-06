use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// Definition for a binary tree node.
///
/// # Note
///
/// I add Ord PartialOrd for sort Vec<TreeNode> when testing
/// Please don't rely on it
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/// Create a binary tree with TreeNode
///
/// # Example
///
/// ```rust
/// use leetcode_prelude::btree;
///
/// let tree = btree![1, 2, 3, null, null, 4, 5];
/// ```
#[macro_export]
macro_rules! btree {
    () => {
        None
    };
    ($($e:expr), *) => {
        {
            use std::rc::Rc;
            use std::cell::RefCell;

            let elems = vec![$(stringify!($e)), *];
            let elems = elems.iter().map(|n| n.parse::<i32>().ok()).collect::<Vec<_>>();
            let head = Some(Rc::new(RefCell::new($crate::TreeNode::new(elems[0].unwrap()))));
            let mut nodes = std::collections::VecDeque::new();
            nodes.push_back(head.as_ref().unwrap().clone());

            for i in elems[1..].chunks(2) {
                let node = nodes.pop_front().unwrap();
                if let Some(val) = i[0]{
                    node.borrow_mut().left = Some(Rc::new(RefCell::new($crate::TreeNode::new(val))));
                    nodes.push_back(node.borrow().left.as_ref().unwrap().clone());
                }
                if i.len() > 1 {
                    if let Some(val) = i[1] {
                        node.borrow_mut().right = Some(Rc::new(RefCell::new($crate::TreeNode::new(val))));
                        nodes.push_back(node.borrow().right.as_ref().unwrap().clone());
                    }
                }
            }
            head
        }
    };
}

// 将 tree 解析为数组
// 由于 null 没法表示所以返回都为字符串
pub fn r_btree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    let mut buf = vec![];
    let mut deque = VecDeque::new();
    if let Some(r) = root {
        deque.push_back(Some(r));
    }
    while !deque.is_empty() {
        let mut ok = false;
        let mut size = deque.len();
        while size > 0 {
            size -= 1;
            match deque.pop_front().unwrap() {
                Some(c) => {
                    buf.push(c.as_ref().borrow().val.to_string());
                    deque.push_back(c.as_ref().borrow().left.clone());
                    deque.push_back(c.as_ref().borrow().right.clone());
                    ok = true;
                }
                None => {
                    buf.push("null".to_string());
                    deque.push_back(None);
                    deque.push_back(None);
                }
            }
        }
        if !ok {
            break;
        }
    }
    let mut ok = false;
    let mut res = vec![];
    for v in buf.iter().rev() {
        if ok || v != "null" {
            ok = true;
            res.push(v.clone());
        }
    }
    res.reverse();
    res
}

#[cfg(test)]
mod tests {
    use crate::r_btree;

    #[test]
    fn test() {
        let btree = btree![-1, 2, 3, null, 2];
        println!("{:#?}", btree);
    }

    #[test]
    fn test2() {
        let btree = btree![-1, 2, 3, null, 2];
        let nums = r_btree(btree);
        println!("{:?}", nums);
    }
}

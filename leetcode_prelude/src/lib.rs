#![feature(uniform_paths)]

mod btree;
mod linkedlist;

pub use btree::TreeNode;
pub use linkedlist::ListNode;
pub use leetcode_test::leetcode_test;

/// Create a Vec<String>
#[macro_export]
macro_rules! vec_string {
    ($($e:expr), *) => {vec![$($e.to_owned()), *]};
}
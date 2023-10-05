use super::commands::new::sanitize_lc_link;
use regex::Regex;
use reqwest::Url;
use std::str::FromStr;

#[test]
fn parse_link_test_1() {
    let out =
        sanitize_lc_link(&"https://leetcode.com/problems/remove-duplicate-letters/".to_owned())
            .unwrap();
    println!("{}", out);
    assert_eq!(
        out,
        Url::from_str("https://leetcode.com/problems/remove-duplicate-letters/").unwrap()
    );
}

#[test]
fn test_regex_for_functions() {
    let re = Regex::new(r"pub\s+fn\s+(?<func>\w+)\s*\(").unwrap();
    let func = re
        .captures(
            "/ Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {",
        )
        .unwrap()["func"]
        .to_owned();
    assert_eq!(func, "add_two_numbers");
}

use super::commands::new::parse_to_simplest_lc_link;
use reqwest::Url;
use std::str::FromStr;

#[test]
fn parse_link_test_1() {
    let out = parse_to_simplest_lc_link(
        &"https://leetcode.com/problems/remove-duplicate-letters/".to_owned(),
    )
    .unwrap();
    println!("{}", out);
    assert_eq!(
        out,
        Url::from_str("https://leetcode.com/problems/remove-duplicate-letters/").unwrap()
    );
}

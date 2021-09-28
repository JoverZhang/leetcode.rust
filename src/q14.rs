pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        // Get shortest prefix
        let mut prefix = &strs[0];
        for str in (&strs)[1..].iter() {
            if str.len() < prefix.len() {
                prefix = str;
            }
        }
        let mut prefix = prefix.to_string();

        for str in strs {
            loop {
                if !str.starts_with(&prefix) {
                    prefix.pop();
                } else {
                    break;
                }
            }
        }
        prefix
    }
}

#[test]
fn test() {
    println!("{:?}", Solution::longest_common_prefix(
        vec![
            "flower".to_owned(),
            "flow".to_owned(),
            "flight".to_owned(),
        ]));
    println!("{:?}", Solution::longest_common_prefix(
        vec![
            "dog".to_owned(),
            "racecar".to_owned(),
            "car".to_owned(),
        ]));
    println!("{:?}", Solution::longest_common_prefix(
        vec![
            "a".to_owned(),
        ]));
    println!("{:?}", Solution::longest_common_prefix(
        vec![
            "".to_owned(),
            "b".to_owned(),
        ]));
}

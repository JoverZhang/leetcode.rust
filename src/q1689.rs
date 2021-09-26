use std::cmp::max;

pub struct Solution {}

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let arr = n.as_bytes();
        let mut rst = 0;
        for x in arr {
            rst = max(rst, x - 48);
        }
        return rst as i32;
    }
}

#[test]
fn test() {
    println!("{}", Solution::min_partitions(String::from("32")));
    println!("{}", Solution::min_partitions(String::from("82734")));
    println!("{}", Solution::min_partitions(String::from("27346209830709182346")));
}

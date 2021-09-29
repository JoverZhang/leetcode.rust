use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut map = HashMap::new();
        map.insert('}', '{');
        map.insert(']', '[');
        map.insert(')', '(');

        let mut stack = vec![0 as char; s.len()];
        let mut i = 1 as usize;

        stack[0] = s.as_bytes()[0] as char;

        for c in s.chars().skip(1) {
            match map.get(&c) {
                Some(partner) => {
                    if i <= 0 {
                        return false;
                    }
                    if stack[i - 1] == *partner {
                        i -= 1;
                    } else {
                        stack[i] = c;
                        i += 1;
                    }
                }
                None => {
                    stack[i] = c;
                    i += 1;
                }
            };
        }
        return i == 0;
    }
}

#[test]
fn test() {
    println!("{}", Solution::is_valid("()".to_owned()));
    println!("{}", Solution::is_valid("()[]{}".to_owned()));
    println!("{}", Solution::is_valid("(]".to_owned()));
    println!("{}", Solution::is_valid("([)]".to_owned()));
    println!("{}", Solution::is_valid("{[]}".to_owned()));
    println!("{}", Solution::is_valid("(])".to_owned()));
}

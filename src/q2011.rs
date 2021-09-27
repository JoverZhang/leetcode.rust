pub struct Solution {}

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut rst = 0;
        for oper in operations {
            if oper.starts_with("-") || oper.ends_with("-") {
                rst -= 1;
            } else if oper.starts_with("+") || oper.ends_with("+") {
                rst += 1;
            };
        }
        return rst;
    }
}

#[test]
fn test() {
    println!("{}", Solution::final_value_after_operations(vec![
        String::from("--X"),
        String::from("X++"),
        String::from("X++"),
    ]));
}

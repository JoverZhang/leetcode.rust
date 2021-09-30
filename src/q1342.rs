pub struct Solution {}

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut n = num;
        let mut rst = 0;
        loop {
            if n == 0 {
                break;
            }

            // event
            if n % 2 == 0 {
                n /= 2;
            }
            // odd
            else {
                n -= 1;
            }

            rst += 1;
        }

        rst
    }
}

#[test]
fn test() {
    println!("{}", Solution::number_of_steps(14));
}

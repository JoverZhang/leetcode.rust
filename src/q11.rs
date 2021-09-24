use std::cmp::{max, min};

pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut arr = vec![0; height.len()];

        let mut l: usize = 0;
        let mut r: usize = arr.len() - 1;

        while l < r {
            let lv = height[l];
            let rv = height[r];
            let contain = min(lv, rv) * (r - l) as i32;
            arr[l] = max(contain, arr[l]);
            arr[r] = max(contain, arr[r]);

            if lv <= rv { l += 1; } else { r -= 1; }
        }

        return *arr.iter().max().unwrap();
    }
}

#[test]
fn test() {
    println!("{}", Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    println!("{}", Solution::max_area(vec![1, 1]));
    println!("{}", Solution::max_area(vec![4, 3, 2, 1, 4]));
    println!("{}", Solution::max_area(vec![1, 2, 1]));
    println!("{}", Solution::max_area(vec![2, 3, 4, 5, 18, 17, 6]));
}

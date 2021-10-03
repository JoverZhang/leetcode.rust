pub struct Solution {}

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let s = s.as_bytes();
        let mut rst = vec![0 as u8; s.len()];
        for i in 0..s.len() {
            rst[indices[i] as usize] = s[i];
        }
        return String::from_utf8_lossy(&*rst).to_string();
    }
}

#[test]
fn test() {
    println!("{}", Solution::restore_string("codeleet".to_owned(), vec![4, 5, 6, 7, 0, 2, 1, 3]));
}

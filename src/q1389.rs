pub struct Solution {}

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut rst = Vec::with_capacity(nums.len());
        for i in 0..nums.len() {
            let idx = index[i] as usize;
            rst.insert(idx, nums[i]);
        }
        return rst;
    }
}

#[test]
fn test() {
    println!("{:?}", Solution::create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]));
    println!("{:?}", Solution::create_target_array(vec![1, 2, 3, 4, 0], vec![0, 1, 2, 3, 0]));
}

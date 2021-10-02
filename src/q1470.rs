pub struct Solution {}

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut rst = Vec::with_capacity(nums.len());
        let arr1 = &nums[..n as usize];
        let arr2 = &nums[n as usize..];

        for i in 0..n as usize {
            rst.push(arr1[i]);
            rst.push(arr2[i]);
        }

        rst
    }
}

#[test]
fn test() {
    println!("{:?}", Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3));
}

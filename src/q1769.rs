pub struct Solution {}

impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let mut arr: Vec<i32> = vec![0; boxes.len()];
        let boxes = boxes.as_bytes();
        for i in 0..boxes.len() {
            if boxes[i] == b'1' {
                let mut l = i as i32 - 1;
                let mut r = i as i32 + 1;

                while l >= 0 || r < boxes.len() as i32 {
                    if l >= 0 {
                        arr[l as usize] += i as i32 - l;
                        l -= 1;
                    }
                    if r < boxes.len() as i32 {
                        arr[r as usize] += r - i as i32;
                        r += 1;
                    }
                }
            }
        }

        arr
    }
}

struct BestSolution {}

impl BestSolution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let (mut sum, mut l, mut r) = (0, 0, 0);
        boxes.chars().enumerate().for_each(|(i, c)| {
            if c == '1' {
                sum += i as i32;
                r += 1
            }
        });

        boxes
            .chars()
            .map(|c| {
                let s = sum;
                if c == '1' {
                    r -= 1;
                    l += 1;
                }
                sum += l;
                sum -= r;
                s
            })
            .collect()
    }
}

#[test]
fn test() {
    println!("{:?}", Solution::min_operations(String::from("110")));
    println!("{:?}", Solution::min_operations(String::from("001011")));

    println!("{:?}", BestSolution::min_operations(String::from("110")));
    println!("{:?}", BestSolution::min_operations(String::from("001011")));
}

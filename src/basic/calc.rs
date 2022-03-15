pub struct Solution {}

impl Solution {

  pub fn add(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
      let sum = a ^ b;
      b = (a & b) << 1;
      a = sum;
    }
    return a;
  }

  pub fn minus(a: i32, b: i32) -> i32 {
    return Solution::add(a, Solution::add(!b, 1));
  }

  pub fn multiple(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b as u32;
    let mut rst = 0;
    while b > 0 {
        if b & 1 == 1 {
            rst = Solution::add(rst, a);
        }
        a <<= 1;
        b >>= 1;
    }
    return rst;
  }

}

#[test]
fn test() {
  assert_eq!(0, Solution::add(0, 0));
  assert_eq!(30, Solution::add(10, 20));
  assert_eq!(126, Solution::add(63, 63));
  assert_eq!(128, Solution::add(64, 64));

  assert_eq!(0, Solution::minus(0, 0));
  assert_eq!(-10, Solution::minus(10, 20));
  assert_eq!(0, Solution::minus(63, 63));
  assert_eq!(-64, Solution::minus(0, 64));

  assert_eq!(0, Solution::multiple(0, 0));
  assert_eq!(200, Solution::multiple(10, 20));
  assert_eq!(3969, Solution::multiple(63, 63));
  assert_eq!(-4096, Solution::multiple(64, -64));
}

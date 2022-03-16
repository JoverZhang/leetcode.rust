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

  fn div(a: i32, b: i32) -> i32 {
    let is_neg = Solution::is_neg(a) ^ Solution::is_neg(b);
    let mut a = if Solution::is_neg(a) { -a } else { a };
    let b = if Solution::is_neg(b) { -b } else { b };
    let mut rst = 0;
    assert!(!Solution::is_neg(a) && !Solution::is_neg(b));

    for i in (0..=30).rev() {
      if (a >> i) >= b {
        rst |= 1 << i;
        a = Solution::minus(a, b << i);
      }
    }
    if is_neg {
      -rst
    } else {
      rst
    }
  }

  pub fn divide(a: i32, b: i32) -> i32 {
    if b == i32::MIN {
      if a == i32::MIN {
        return 1;
      }
      return 0;
    }
    if a == i32::MIN {
      let rst = Solution::div(a + 1, b);
      if rst == i32::MAX {
        return i32::MAX;
      }
      return rst + Solution::div(Solution::minus(a, Solution::multiple(rst, b)), b);
    }
    Solution::div(a, b)
  }

  fn is_neg(n: i32) -> bool {
    n < 0
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

  assert_eq!(3, Solution::divide(9, 3));
  assert_eq!(-3, Solution::divide(9, -3));
  assert_eq!(3, Solution::divide(10, 3));
  assert_eq!(-2, Solution::divide(7, -3));
  assert_eq!(0, Solution::divide(i32::MAX, i32::MIN));
  assert_eq!(1, Solution::divide(i32::MIN, i32::MIN));
  assert_eq!(i32::MAX, Solution::divide(i32::MIN, -1));
}

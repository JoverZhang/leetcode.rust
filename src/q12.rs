pub struct Solution {}

impl Solution {
    fn parse_1(num: i32) -> &'static str {
        if num == 1 {
            return "I";
        } else if num == 2 {
            return "II";
        } else if num == 3 {
            return "III";
        } else if num == 4 {
            return "IV";
        } else if num == 5 {
            return "V";
        } else if num == 6 {
            return "VI";
        } else if num == 7 {
            return "VII";
        } else if num == 8 {
            return "VIII";
        } else if num == 9 {
            return "IX";
        }
        ""
    }

    fn parse_2(num: i32) -> &'static str {
        if num == 1 {
            return "X";
        } else if num == 2 {
            return "XX";
        } else if num == 3 {
            return "XXX";
        } else if num == 4 {
            return "XL";
        } else if num == 5 {
            return "L";
        } else if num == 6 {
            return "LX";
        } else if num == 7 {
            return "LXX";
        } else if num == 8 {
            return "LXXX";
        } else if num == 9 {
            return "XC";
        }
        ""
    }

    fn parse_3(num: i32) -> &'static str {
        if num == 1 {
            return "C";
        } else if num == 2 {
            return "CC";
        } else if num == 3 {
            return "CCC";
        } else if num == 4 {
            return "CD";
        } else if num == 5 {
            return "D";
        } else if num == 6 {
            return "DC";
        } else if num == 7 {
            return "DCC";
        } else if num == 8 {
            return "DCCC";
        } else if num == 9 {
            return "CM";
        }
        ""
    }

    fn parse_4(num: i32) -> &'static str {
        if num == 1 {
            return "M";
        } else if num == 2 {
            return "MM";
        } else if num == 3 {
            return "MMM";
        }
        ""
    }

    pub fn int_to_roman(num: i32) -> String {
        let mut rst = String::new();
        if num >= 1000 {
            rst += Solution::parse_4((num / 1000) % 10);
        }
        if num >= 100 {
            rst += Solution::parse_3((num / 100) % 10);
        }
        if num >= 10 {
            rst += Solution::parse_2((num / 10) % 10);
        }
        if num >= 1 {
            rst += Solution::parse_1(num % 10);
        }
        rst
    }
}

struct BestSolution {}

impl BestSolution {
    pub fn int_to_roman(num: i32) -> String {
        let symbols = [
            ("M", 1000),
            ("CM", 900),
            ("D", 500),
            ("CD", 400),
            ("C", 100),
            ("XC", 90),
            ("L", 50),
            ("XL", 40),
            ("X", 10),
            ("IX", 9),
            ("V", 5),
            ("IV", 4),
            ("I", 1),
        ];
        let mut rst = String::new();
        let mut num_ = num;

        for (code, value) in symbols.iter() {
            let multiplier = num_ / value;
            num_ -= value * multiplier;
            rst.push_str(&code.repeat(multiplier as usize))
        }
        rst
    }
}

#[test]
fn test() {
    println!("{}", Solution::int_to_roman(3));
    println!("{}", Solution::int_to_roman(4));
    println!("{}", Solution::int_to_roman(9));
    println!("{}", Solution::int_to_roman(58));
    println!("{}", Solution::int_to_roman(1994));

    println!("{}", BestSolution::int_to_roman(3));
    println!("{}", BestSolution::int_to_roman(4));
    println!("{}", BestSolution::int_to_roman(9));
    println!("{}", BestSolution::int_to_roman(58));
    println!("{}", BestSolution::int_to_roman(1994));
}


pub struct Solution {}

impl Solution {
    pub fn interpret(command: String) -> String {
        let command = command.as_bytes();
        let mut rst = "".to_owned();
        let mut i = 0;
        while i < command.len() {
            if command[i] == 'G' as u8 {
                rst += "G";
                i += 1;
            } else if command[i..i + 2] == *"()".as_bytes() {
                rst += "o";
                i += 2;
            } else if command[i..i + 4] == *"(al)".as_bytes() {
                rst += "al";
                i += 4;
            } else {
                println!("err");
                break;
            }
        }
        return rst;
    }
}

pub struct BestSolution {}

impl BestSolution {
    pub fn interpret(command: String) -> String {
        command.replace("()", "o").replace("(al)", "al")
    }
}

#[test]
fn test() {
    println!("{}", Solution::interpret("G()(al)".to_owned()));
    println!("{}", Solution::interpret("G()()()()(al)".to_owned()));
    println!("{}", Solution::interpret("(al)G(al)()()G".to_owned()));

    println!("{}", BestSolution::interpret("G()(al)".to_owned()));
    println!("{}", BestSolution::interpret("G()()()()(al)".to_owned()));
    println!("{}", BestSolution::interpret("(al)G(al)()()G".to_owned()));
}

//! https://leetcode.com/problems/shuffle-string/

pub struct Solution {}

impl Solution {
    pub fn interpret(command: String) -> String {
        let mut rest_command = command.as_str();
        let mut result = String::new();
        while rest_command.len() > 0 {
            if rest_command.starts_with('G') {
                result.push('G');
                rest_command = &rest_command[1..];
            } else if rest_command.starts_with("()") {
                result.push('o');
                rest_command = &rest_command[2..];
            } else if rest_command.starts_with("(al)") {
                result.push_str("al");
                rest_command = &rest_command[4..];
            } else {
                panic!();
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

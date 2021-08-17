//! https://leetcode.com/problems/shuffle-string/

pub struct Solution {}

impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut result = String::with_capacity(s.len());
        let mut words = vec![""; 10];
        let mut max_index:usize = 0;
        for word in s.split(' ') {
            let last_char:char = word.chars().last().unwrap();
            let index = (last_char.to_digit(10).unwrap() - 1) as usize;
            words[index] = &word[0..word.len()-1];
            
            if (index > max_index) {
                max_index = index;
            }
        }
        for i in 0..max_index + 1 {
            result.push_str(words[i]);
            if i < max_index {
                result.push(' ');
            }
        }
        result
    }
}


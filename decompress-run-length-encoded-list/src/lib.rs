//! https://leetcode.com/problems/shuffle-string/

pub struct Solution {}

impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        assert!(nums.len() % 2 == 0);
        let mut iter = nums.iter();
        loop {
            let freq = iter.next();
            let val = iter.next();
            match (freq, val) {
                (Some(f), Some(v)) => {
                    for i in 0..*f {
                        result.push(*v);
                    }
                },
                _ => break,
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn example_1() {
        assert_eq!(
            Solution::decompress_rl_elist(vec![1, 2, 3, 4]),
            vec![2, 4, 4, 4]);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::decompress_rl_elist([1, 1, 2, 3]),
            vec![1, 3, 3]);
    }
}

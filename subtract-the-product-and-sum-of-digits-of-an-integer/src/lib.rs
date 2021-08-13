//! https://leetcode.com/problems/shuffle-string/

pub struct Solution {}

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut product = 1_i32;
        let mut sum = 0_i32;
        let mut n = n;
        while n > 0 {
            product *= n % 10;
            sum += n % 10;
            n /= 10;
        }
        product - sum
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example() {
        assert_eq!(Solution::subtract_product_and_sum(234), 15);
        assert_eq!(Solution::subtract_product_and_sum(4421), 21);
    }
}

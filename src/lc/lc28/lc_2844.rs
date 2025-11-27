// https://leetcode.com/problems/minimum-operations-to-make-a-special-number/
// 2844. Minimum Operations to Make a Special Number
pub struct Solution;
impl Solution {
    pub fn minimum_operations(num: String) -> i32 {
        let n = num.len() as i32;
        let mut found0 = false;
        let mut found5 = false;
        for (i, c) in num.chars().rev().enumerate() {
            match c {
                '0' => {
                    if found0 {
                        return (i - 1) as i32;
                    }
                    found0 = true;
                }
                '5' => {
                    if found0 {
                        return (i - 1) as i32;
                    }
                    found5 = true;
                }
                '2' | '7' => {
                    if found5 {
                        return (i - 1) as i32;
                    }
                }
                _ => (),
            }
        }
        if found0 {
            n - 1
        } else {
            n
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_operations() {
        assert_eq!(Solution::minimum_operations("2245047".to_string()), 2);
        assert_eq!(Solution::minimum_operations("2908305".to_string()), 3);
        assert_eq!(Solution::minimum_operations("10".to_string()), 1);
    }
}

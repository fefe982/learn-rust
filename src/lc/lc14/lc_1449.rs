// https://leetcode.com/problems/form-largest-integer-with-digits-that-add-up-to-target/
// 1449. Form Largest Integer With Digits That Add up to Target
pub struct Solution;
impl Solution {
    fn get_number(cost: &Vec<i32>, idx: usize, target: i32, dp: &mut Vec<Vec<String>>) -> String {
        if target == 0 {
            return "".to_string();
        }
        if target < 0 || idx > 9 {
            return "0".to_string();
        }
        if dp[idx][target as usize] != "" {
            return dp[idx][target as usize].clone();
        }
        let mut inc = Self::get_number(cost, 1, target - cost[idx - 1], dp);
        if inc != "0" {
            inc.push(char::from(idx as u8 + b'0'));
        }
        let exc = Self::get_number(cost, idx + 1, target, dp);
        let res = if inc == "0" {
            exc
        } else if exc == "0" {
            inc
        } else if inc.len() > exc.len() || (inc.len() == exc.len() && inc > exc) {
            inc
        } else {
            exc
        };
        dp[idx][target as usize] = res.clone();
        res
    }
    pub fn largest_number(cost: Vec<i32>, target: i32) -> String {
        Self::get_number(
            &cost,
            1,
            target,
            &mut vec![vec!["".to_string(); target as usize + 1]; 10],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_largest_number() {
        assert_eq!(Solution::largest_number(vec![4, 3, 2, 5, 6, 7, 2, 5, 5], 9), "7772");
        assert_eq!(Solution::largest_number(vec![7, 6, 5, 5, 5, 6, 8, 7, 8], 12), "85");
        assert_eq!(Solution::largest_number(vec![2, 4, 6, 2, 4, 6, 4, 4, 4], 5), "0");
    }
}

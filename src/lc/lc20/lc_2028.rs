// https://leetcode.com/problems/find-missing-observations/
// 2028. Find Missing Observations
pub struct Solution;
impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let sum = rolls.iter().sum::<i32>();
        let nr = rolls.len() as i32;
        let mut diff = mean * (n + nr) - sum;
        let left = n;
        if diff < left || diff > left * 6 {
            return vec![];
        }
        let mut res = vec![];
        for i in 0..left {
            res.push(diff / (left - i));
            diff -= res[i as usize];
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(rolls: &[i32], mean: i32, n: i32, expect: bool) {
        let res = Solution::missing_rolls(rolls.to_vec(), mean, n);
        if expect {
            assert_eq!(res.len(), n as usize);
            assert_eq!(
                rolls.iter().chain(res.iter()).cloned().sum::<i32>(),
                mean * (n + rolls.len() as i32)
            );
            for r in res {
                assert!(r >= 1 && r <= 6);
            }
        } else {
            assert!(res.is_empty());
        }
    }
    #[test]
    fn test_missing_rolls() {
        check(&[3, 2, 4, 3], 4, 2, true);
        check(&[1, 5, 6], 3, 4, true);
        check(&[1, 2, 3, 4], 6, 4, false);
    }
}

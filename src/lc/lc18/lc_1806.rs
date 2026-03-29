// https://leetcode.com/problems/minimum-number-of-operations-to-reinitialize-a-permutation/
// 1806. Minimum Number of Operations to Reinitialize a Permutation
pub struct Solution;
impl Solution {
    pub fn reinitialize_permutation(n: i32) -> i32 {
        if n == 2 {
            return 1;
        }

        let mut pos = 1;
        let mut steps = 0;

        loop {
            pos = if pos < n / 2 {
                pos * 2
            } else {
                (pos - n / 2) * 2 + 1
            };

            steps += 1;
            if pos == 1 {
                break;
            }
        }

        steps
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reinitialize_permutation() {
        assert_eq!(Solution::reinitialize_permutation(2), 1);
        assert_eq!(Solution::reinitialize_permutation(4), 2);
        assert_eq!(Solution::reinitialize_permutation(6), 4);
    }
}

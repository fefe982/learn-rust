// https://leetcode.com/problems/minimum-operations-to-form-subsequence-with-target-sum/
// 2835. Minimum Operations to Form Subsequence With Target Sum
pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, target: i32) -> i32 {
        let mut cnt = vec![0; 31];
        for i in nums {
            cnt[i.trailing_zeros() as usize] += 1;
        }
        let mut res = 0;
        for i in 0..31 {
            if i > 0 {
                cnt[i] += cnt[i - 1] / 2;
            }
            if target & (1 << i) != 0 {
                if cnt[i] == 0 {
                    for j in i..31 {
                        if cnt[j] > 0 {
                            cnt[j] -= 1;
                            res += j - i;
                            break;
                        } else {
                            cnt[j] = 1;
                            if j == 30 {
                                return -1;
                            }
                        }
                    }
                } else {
                    cnt[i] -= 1;
                }
            }
        }
        res as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations(vec![1, 2, 8], 7), 1);
        assert_eq!(Solution::min_operations(vec![1, 32, 1, 2], 12), 2);
        assert_eq!(Solution::min_operations(vec![1, 32, 1], 35), -1);
    }
}

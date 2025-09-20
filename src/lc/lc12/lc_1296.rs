// https://leetcode.com/problems/divide-array-in-sets-of-k-consecutive-numbers/
// 1296. Divide Array in Sets of K Consecutive Numbers
pub struct Solution;
impl Solution {
    pub fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        if nums.len() % k != 0 {
            return false;
        }
        if k == 1 {
            return true;
        }
        let mut nums = nums;
        nums.sort_unstable();
        let mut v = std::collections::VecDeque::new();
        let mut expect = 0;
        let mut expect_cnt = 0;
        for i in nums {
            if expect_cnt == 0 {
                expect = i + 1;
                expect_cnt = 1;
                v.push_back(i);
            } else if i == expect - 1 {
                expect_cnt += 1;
                v.push_back(i);
            } else if i == expect {
                expect_cnt -= 1;
                if expect_cnt == 0 {
                    expect += 1;
                    while let Some(&j) = v.front() {
                        if expect - j == k as i32 {
                            v.pop_front();
                        } else {
                            break;
                        }
                    }
                    expect_cnt = v.len();
                }
            } else {
                return false;
            }
        }
        v.is_empty()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_possible_divide() {
        assert_eq!(Solution::is_possible_divide(vec![1, 1, 2, 2, 3, 3], 2), false);
        assert_eq!(Solution::is_possible_divide(vec![1, 2, 3, 3, 4, 4, 5, 6], 4), true);
        assert_eq!(
            Solution::is_possible_divide(vec![3, 2, 1, 2, 3, 4, 3, 4, 5, 9, 10, 11], 3),
            true
        );
        assert_eq!(Solution::is_possible_divide(vec![1, 2, 3, 4], 3), false);
    }
}

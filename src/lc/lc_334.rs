// https://leetcode.com/problems/increasing-triplet-subsequence/
// 334. Increasing Triplet Subsequence
pub struct Solution;
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut v = vec![];
        for num in nums {
            if v.is_empty() || num > v[v.len() - 1] {
                v.push(num);
                if v.len() == 3 {
                    return true;
                }
            } else {
                for i in 0..v.len() {
                    if v[i] >= num {
                        v[i] = num;
                        break;
                    }
                }
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn increasing_triplet() {
        assert_eq!(Solution::increasing_triplet(vec![20, 100, 10, 12, 5, 13]), true);
        assert_eq!(Solution::increasing_triplet(vec![1, 2, 3, 4, 5]), true);
        assert_eq!(Solution::increasing_triplet(vec![5, 4, 3, 2, 1]), false);
        assert_eq!(Solution::increasing_triplet(vec![2, 1, 5, 0, 4, 6]), true);
    }
}

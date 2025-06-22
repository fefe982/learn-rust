// https://leetcode.com/problems/permutations/
// 46. Permutations
pub struct Solution;
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut res = vec![nums.clone()];
        loop {
            let mut last = nums.pop().unwrap();
            let mut q = std::collections::BinaryHeap::new();
            while let Some(&n) = nums.last() {
                q.push(std::cmp::Reverse(last));
                if n < last {
                    q.push(std::cmp::Reverse(n));
                    let mut idx = nums.len() - 1;
                    while let Some(qn) = q.pop() {
                        if qn.0 > n && idx != usize::MAX {
                            nums[idx] = qn.0;
                            idx = usize::MAX;
                        } else {
                            nums.push(qn.0);
                        }
                    }
                    res.push(nums.clone());
                    break;
                } else {
                    last = nums.pop().unwrap();
                }
            }
            if nums.is_empty() {
                return res;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn my_pow() {
        assert_eq!(
            Solution::permute(vec![1, 2, 3]),
            vec_vec![[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]
        );
        assert_eq!(Solution::permute(vec![0, 1]), vec_vec![[0, 1], [1, 0]]);
        assert_eq!(Solution::permute(vec![1]), vec_vec![[1]]);
    }
}

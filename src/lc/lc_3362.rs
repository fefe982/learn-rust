// https://leetcode.com/problems/zero-array-transformation-iii/
// 3362. Zero Array Transformation III
pub struct Solution;
impl Solution {
    pub fn max_removal(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut d = vec![0; nums.len() + 1];
        let mut queries = queries;
        queries.sort_unstable();
        let mut diff = 0;
        let mut h = std::collections::BinaryHeap::new();
        let mut j = 0;
        for i in 0..nums.len() {
            diff += d[i];
            while j < queries.len() && queries[j][0] as usize <= i {
                if queries[j][0] as usize <= i {
                    h.push(queries[j][1] as usize + 1);
                    j += 1;
                } else {
                    break;
                }
            }
            while diff < nums[i] {
                if let Some(q) = h.pop() {
                    if q <= i {
                        return -1;
                    }
                    diff += 1;
                    d[q] -= 1;
                } else {
                    return -1;
                }
            }
        }
        h.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_removal() {
        assert_eq!(
            Solution::max_removal(vec![2, 0, 2], vec_vec![[0, 2], [0, 2], [1, 1]]),
            1
        );
        assert_eq!(
            Solution::max_removal(vec![1, 1, 1, 1], vec_vec![[1, 3], [0, 2], [1, 3], [1, 2]]),
            2
        )
    }
}

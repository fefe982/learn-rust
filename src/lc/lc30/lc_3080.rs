// https://leetcode.com/problems/mark-elements-on-array-by-performing-queries/
// 3080. Mark Elements on Array by Performing Queries
pub struct Solution;
impl Solution {
    pub fn unmarked_sum_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let mut hp = std::collections::BinaryHeap::new();
        let mut sum = 0;
        for i in 0..nums.len() {
            hp.push(std::cmp::Reverse((nums[i], i)));
            sum += nums[i] as i64;
        }
        let mut nums = nums;
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            let (idx, mut cnt) = (q[0] as usize, q[1]);
            sum -= nums[idx] as i64;
            nums[idx] = 0;
            while cnt > 0 {
                if let Some(std::cmp::Reverse((num, i))) = hp.pop() {
                    if nums[i] == 0 {
                        continue;
                    }
                    nums[i] = 0;
                    sum -= num as i64;
                    cnt -= 1;
                } else {
                    break;
                }
            }
            ans.push(sum);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn unmarked_sum_array() {
        assert_eq!(
            Solution::unmarked_sum_array(vec![1, 2, 2, 1, 2, 3, 1], vec_vec![[1, 2], [3, 3], [4, 2]]),
            vec![8, 3, 0]
        );
        assert_eq!(
            Solution::unmarked_sum_array(vec![1, 4, 2, 3], vec_vec![[0, 1]]),
            vec![7]
        );
    }
}

// https://leetcode.com/problems/find-score-of-an-array-after-marking-all-elements/
// 2593. Find Score of an Array After Marking All Elements
pub struct Solution;
impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let mut nums = nums
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v as i64, i))
            .collect::<Vec<_>>();
        let mut marked = vec![false; nums.len()];
        nums.sort();
        let mut sum = 0;
        for (n, i) in nums.into_iter() {
            if !marked[i] {
                marked[i] = true;
                if i > 0 {
                    marked[i - 1] = true;
                }
                if i + 1 < marked.len() {
                    marked[i + 1] = true;
                }
                sum += n;
            }
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_score() {
        assert_eq!(Solution::find_score(vec![2, 1, 3, 4, 5, 2]), 7);
        assert_eq!(Solution::find_score(vec![2, 3, 5, 1, 3, 2]), 5);
    }
}

// https://leetcode.com/problems/subsets/
// 78. Subsets
pub struct Solution;
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![]];
        let mut nums = nums;
        nums.sort_unstable();
        let mut i = 0;
        while i < nums.len() {
            let mut dup = ans
                .iter()
                .map(|x| {
                    let mut x = x.clone();
                    x.push(nums[i]);
                    x
                })
                .collect::<Vec<_>>();
            let mut j = i + 1;
            while j < nums.len() && nums[j] == nums[i] {
                let ddup = dup
                    .iter()
                    .map(|x| {
                        let mut x = x.clone();
                        x.push(nums[i]);
                        x
                    })
                    .collect::<Vec<_>>();
                ans.append(&mut dup);
                dup = ddup;
                j += 1;
            }
            ans.append(&mut dup);
            i = j;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_subsets() {
        assert_eq!(
            Solution::subsets(vec![1, 2, 3]),
            vec_vec![[], [1], [2], [1, 2], [3], [1, 3], [2, 3], [1, 2, 3]]
        );
        assert_eq!(Solution::subsets(vec![0]), vec_vec![[], [0]]);
    }
}

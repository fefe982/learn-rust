// https://leetcode.com/problems/count-increasing-quadruplets/
// 2552. Count Increasing Quadruplets
pub struct Solution;
impl Solution {
    fn insert(tree: &mut Vec<i32>, v: i32) {
        let mut v = v as usize;
        while v < tree.len() {
            tree[v] += 1;
            v += v & (!v + 1);
        }
    }
    fn count(tree: &Vec<i32>, v: i32) -> i32 {
        let mut v = v as usize;
        let mut ans = 0;
        while v > 0 {
            ans += tree[v];
            v -= v & (!v + 1);
        }
        ans
    }
    pub fn count_quadruplets(nums: Vec<i32>) -> i64 {
        let len = nums.len();
        let mut tree = vec![0; len];
        let mut ans = 0;
        let last = nums[nums.len() - 1];
        Self::insert(&mut tree, nums[0]);
        for j in 1..nums.len() - 2 {
            Self::insert(&mut tree, nums[j]);
            let mut cl = 0;
            if last > nums[j] {
                cl += 1;
            }
            for k in (j + 1..nums.len() - 1).rev() {
                if nums[k] > nums[j] {
                    cl += 1;
                    continue;
                }
                ans += Self::count(&tree, nums[k]) as i64 * cl as i64;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_quadruplets() {
        assert_eq!(Solution::count_quadruplets(vec![1, 3, 2, 4, 5]), 2);
        assert_eq!(Solution::count_quadruplets(vec![1, 2, 3, 4]), 0);
    }
}

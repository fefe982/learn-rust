// https://leetcode.com/problems/intersection-of-two-arrays-ii/
// 350. Intersection of Two Arrays II
pub struct Solution;
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut cnt = vec![0; 1001];
        for n in nums1 {
            cnt[n as usize] += 1;
        }
        let mut ans = vec![];
        for n in nums2 {
            if cnt[n as usize] > 0 {
                ans.push(n);
                cnt[n as usize] -= 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(nums1: Vec<i32>, nums2: Vec<i32>, mut expect: Vec<i32>) {
        let mut ans = Solution::intersect(nums1, nums2);
        ans.sort_unstable();
        expect.sort_unstable();
        assert_eq!(ans, expect);
    }
    #[test]
    fn test_intersect() {
        check(vec![1, 2, 2, 1], vec![2, 2], vec![2, 2]);
        check(vec![4, 9, 5], vec![9, 4, 9, 8, 4], vec![4, 9]);
    }
}

// https://leetcode.com/problems/split-and-merge-array-transformation/
// 3690. Split and Merge Array Transformation
pub struct Solution;
impl Solution {
    pub fn min_split_merge(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        if nums1 == nums2 {
            return 0;
        }
        let mut m = std::collections::HashSet::new();
        m.insert(nums1.clone());
        let mut q = std::collections::VecDeque::new();
        q.push_back((nums1.clone(), 0));
        while let Some((nums, steps)) = q.pop_front() {
            for l in 1..nums.len() {
                for s in 0..nums.len() + 1 - l {
                    let e = s + l;
                    let mut t = nums.clone();
                    t[0..l].copy_from_slice(&nums[s..e]);
                    t[l..s + l].copy_from_slice(&nums[0..s]);
                    if !m.contains(&t) {
                        if t == nums2 {
                            return steps + 1;
                        }
                        m.insert(t.clone());
                        q.push_back((t.clone(), steps + 1));
                    }
                    for p in 1..nums.len() + 1 - l {
                        t[p - 1] = t[p + l - 1];
                        t[p..p + l].copy_from_slice(&nums[s..e]);
                        if m.contains(&t) {
                            continue;
                        }
                        if t == nums2 {
                            return steps + 1;
                        }
                        m.insert(t.clone());
                        q.push_back((t.clone(), steps + 1));
                    }
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_split_merge() {
        assert_eq!(Solution::min_split_merge(vec![3, 1, 2], vec![1, 2, 3]), 1);
        assert_eq!(
            Solution::min_split_merge(vec![1, 1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1, 1]),
            3
        );
    }
}

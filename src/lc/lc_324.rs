// https://leetcode.com/problems/wiggle-sort-ii/
// 324. Wiggle Sort II
pub struct Solution;
impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let l = nums.len();
        let (_, &mut n, _) = nums.select_nth_unstable(l / 2);
        let idx = |i: usize| (1 + 2 * i) % (l | 1);
        let mut i = 0;
        let mut j = 0;
        let mut k = l - 1;
        while j <= k {
            match nums[idx(j)].cmp(&n) {
                std::cmp::Ordering::Greater => {
                    nums.swap(idx(i), idx(j));
                    i += 1;
                    j += 1
                }
                std::cmp::Ordering::Equal => j += 1,
                std::cmp::Ordering::Less => {
                    nums.swap(idx(j), idx(k));
                    k -= 1;
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(mut nums: Vec<i32>) {
        let mut origin = nums.clone();
        Solution::wiggle_sort(&mut nums);
        for i in 0..nums.len() - 1 {
            if i % 2 == 0 {
                assert!(nums[i] < nums[i + 1]);
            } else {
                assert!(nums[i] > nums[i + 1]);
            }
        }
        nums.sort();
        origin.sort();
        assert_eq!(nums, origin);
    }
    #[test]
    fn wiggle_sort() {
        check(vec![1, 5, 1, 1, 6, 4]);
        check(vec![1, 3, 2, 2, 3, 1]);
    }
}

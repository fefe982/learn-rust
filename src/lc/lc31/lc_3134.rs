// https://leetcode.com/problems/find-the-median-of-the-uniqueness-array/
// 3134. Find the Median of the Uniqueness Array
pub struct Solution;
impl Solution {
    fn count(nums: &Vec<i32>, bound: usize) -> i64 {
        let mut m = std::collections::HashMap::<i32, i32>::new();
        let mut ans = 0;
        let mut i = 0;
        let mut j = 0;
        while j < nums.len() {
            while j < nums.len() && m.len() <= bound {
                m.entry(nums[j]).and_modify(|e| *e += 1).or_insert(1);
                j += 1;
            }
            if m.len() > bound {
                while m.len() > bound {
                    ans += (j - i - 1) as i64;
                    m.entry(nums[i]).and_modify(|e| *e -= 1);
                    if m[&nums[i]] == 0 {
                        m.remove(&nums[i]);
                    }
                    i += 1;
                }
            }
        }
        ans + (j - i + 1) as i64 * (j - i) as i64 / 2
    }
    pub fn median_of_uniqueness_array(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len();
        let limit = (nums.len() as i64 * (nums.len() + 1) as i64 / 2 + 1) / 2;
        while l + 1 < r {
            let m = (l + r) / 2;
            if Self::count(&nums, m) >= limit {
                r = m;
            } else {
                l = m;
            }
        }
        r as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_median_of_uniqueness_array() {
        assert_eq!(Solution::median_of_uniqueness_array(vec![1, 2, 3]), 1);
        assert_eq!(Solution::median_of_uniqueness_array(vec![3, 4, 3, 4, 5]), 2);
        assert_eq!(Solution::median_of_uniqueness_array(vec![4, 3, 5, 4]), 2);
    }
}

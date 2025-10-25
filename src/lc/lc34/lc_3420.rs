// https://leetcode.com/problems/count-non-decreasing-subarrays-after-k-operations/
// 3420. Count Non-Decreasing Subarrays After K Operations
pub struct Solution;
impl Solution {
    pub fn count_non_decreasing_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let mut cnt = 0;
        let mut kk = 0;
        let mut l = std::collections::VecDeque::new();
        (&mut l).push_back((i32::MAX, nums.len()));
        for i in (0..nums.len()).rev() {
            while let Some((h, ih)) = l.pop_back() {
                if nums[i] >= h {
                    kk += (nums[i] - h) as i64 * (l.back().unwrap().1 - ih) as i64;
                } else {
                    l.push_back((h, ih));
                    l.push_back((nums[i], i));
                    break;
                }
            }
            let (_, mut ie) = l.pop_front().unwrap();
            while kk > k as i64 {
                ie -= 1;
                let &(lh, lie) = l.front().unwrap();
                kk -= (lh - nums[ie]) as i64;
                if lie == ie {
                    l.pop_front();
                }
            }
            l.push_front((i32::MAX, ie));
            cnt += (nums.len() - ie) as i64;
        }
        let len = nums.len() as i64;
        len * (len + 1) / 2 - cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_non_decreasing_subarrays() {
        assert_eq!(Solution::count_non_decreasing_subarrays(vec![6, 3, 1, 2, 4, 4], 7), 17);
        assert_eq!(Solution::count_non_decreasing_subarrays(vec![6, 3, 1, 3, 6], 4), 12);
    }
}

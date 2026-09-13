// https://leetcode.com/problems/count-shadow-pairs-ii/
// 4055. Count Shadow Pairs II
pub struct Solution;
impl Solution {
    fn count(a: &[i32], low: i32, high: i32) -> i32 {
        let n = a.len();
        if n <= 1 || low + 1 == high {
            return 0;
        }
        let mut ans = 0;
        let mid = low + (high - low) / 2;
        let mut b = vec![];
        let mut c = vec![];
        let mut low_s = vec![];
        let mut high_s = vec![];
        for i in 0..n {
            if a[i] < mid {
                while let Some(&l) = low_s.last() {
                    if a[l] < a[i] {
                        low_s.pop();
                    } else {
                        break;
                    }
                }
                low_s.push(i);
                b.push(a[i]);
            } else {
                while let Some(&h) = high_s.last() {
                    if a[h] >= a[i] {
                        high_s.pop();
                    } else {
                        break;
                    }
                }
                ans += low_s.len() as i32;
                if let Some(&h) = high_s.last() {
                    ans -= low_s.partition_point(|&l| l < h) as i32;
                }
                high_s.push(i);
                c.push(a[i]);
            }
        }
        ans += Self::count(&b, low, mid);
        ans += Self::count(&c, mid, high);
        ans
    }
    pub fn shadow_pairs(nums: Vec<i32>) -> i32 {
        let mut m = std::collections::BTreeMap::new();
        for &n in &nums {
            m.insert(n, 0);
        }
        for ((_, v), i) in m.iter_mut().zip(0..) {
            *v = i;
        }
        let mut nums = nums;
        for v in nums.iter_mut() {
            *v = *m.get(v).unwrap();
        }
        Self::count(&nums, 0, m.len() as i32)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_shadow_pairs_ii() {
        assert_eq!(Solution::shadow_pairs(vec![3, 1, 4, 2, 5]), 5);
        assert_eq!(Solution::shadow_pairs(vec![6, 7, 8, 9]), 3);
    }
}

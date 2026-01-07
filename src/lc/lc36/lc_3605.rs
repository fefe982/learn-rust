// https://leetcode.com/problems/minimum-stability-factor-of-array/
// 3605. Minimum Stability Factor of Array
pub struct Solution;
impl Solution {
    pub fn min_stable(nums: Vec<i32>, max_c: i32) -> i32 {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            loop {
                if a == 0 {
                    return b;
                }
                b %= a;
                if b == 0 {
                    return a;
                }
                a %= b;
            }
        }
        let mut nums = nums;
        let mut left = vec![0; nums.len()];
        let mut l = 0;
        let mut b = 0;
        let mut g = 0;
        let mut nn = 0;
        for r in 0..nums.len() {
            if nums[r] > 1 {
                nn += 1;
            }
            g = gcd(g, nums[r]);
            while l < r && gcd(nums[l], g) == 1 {
                if b <= l {
                    for j in (l + 1..r).rev() {
                        nums[j] = gcd(nums[j], nums[j + 1]);
                        if nums[j] == 1 {
                            l = j;
                            break;
                        }
                    }
                    b = r;
                    g = 0;
                }
                l += 1;
            }
            left[r] = l
        }
        if max_c >= nn {
            return 0;
        }
        let check = |len: usize| -> bool {
            let mut m = 0;
            let mut i = len;
            while i < nums.len() {
                if i - left[i] >= len {
                    m += 1;
                    i += len + 1;
                } else {
                    i += 1;
                }
            }
            m <= max_c
        };
        if check(1) {
            return 1;
        }
        let mut l = 1;
        let mut r = nums.len();
        while l + 1 < r {
            let m = (l + r) / 2;
            if check(m) {
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
    fn min_stable() {
        assert_eq!(Solution::min_stable(vec![3, 5, 10], 1), 1);
        assert_eq!(Solution::min_stable(vec![2, 6, 8], 2), 1);
        assert_eq!(Solution::min_stable(vec![2, 4, 9, 6], 1), 2);
    }
}

// https://leetcode.com/problems/find-a-value-of-a-mysterious-function-closest-to-target/
// 1521. Find a Value of a Mysterious Function Closest to Target
pub struct Solution;
impl Solution {
    fn update(cnt: &mut Vec<i32>, mut n: i32, off: i32) {
        let mut i = 0;
        while n > 0 {
            if n & 1 == 1 {
                cnt[i as usize] += off;
            }
            n >>= 1;
            i += 1;
        }
    }
    fn get(cnt: &Vec<i32>, c: i32) -> i32 {
        let mut ans = 0;
        for i in 0..32 {
            if cnt[i] >= c {
                ans += 1 << i;
            }
        }
        ans
    }
    pub fn closest_to_target(arr: Vec<i32>, target: i32) -> i32 {
        let mut min = i32::MAX;
        let mut cnt = vec![0; 32];
        Self::update(&mut cnt, arr[0], 1);
        let mut l = 0;
        let mut r = 1;
        loop {
            let num = Self::get(&cnt, (r - l) as i32);
            min = min.min((num - target).abs());
            if min == 0 {
                return 0;
            }
            if num < target {
                Self::update(&mut cnt, arr[l], -1);
                l += 1;
            }
            if l == r || num > target {
                if r == arr.len() {
                    break;
                } else {
                    Self::update(&mut cnt, arr[r], 1);
                    r += 1;
                }
            }
        }
        min
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_closest_to_target() {
        assert_eq!(Solution::closest_to_target(vec![9, 12, 3, 7, 15], 5), 2);
        assert_eq!(Solution::closest_to_target(vec![1000000, 1000000, 1000000], 1), 999999);
        assert_eq!(Solution::closest_to_target(vec![1, 2, 4, 8, 16], 0), 0);
    }
}

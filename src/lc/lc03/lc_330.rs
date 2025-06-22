// https://leetcode.com/problems/patching-array/
// 330. Patching Array
pub struct Solution;
impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let n = n as i64;
        let mut s = 0i64;
        let mut cnt = 0;
        for i in nums {
            let ii = i as i64;
            while ii > s + 1 && s < n {
                cnt += 1;
                s += s + 1;
            }
            s += ii;
            if s >= n {
                return cnt;
            }
        }
        while s < n {
            cnt += 1;
            s += s + 1;
        }
        cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_patches() {
        assert_eq!(
            Solution::min_patches(
                vec![1, 2, 2, 6, 34, 38, 41, 44, 47, 47, 56, 59, 62, 73, 77, 83, 87, 89, 94],
                20
            ),
            1
        );
        assert_eq!(Solution::min_patches(vec![1, 3], 6), 1);
        assert_eq!(Solution::min_patches(vec![1, 5, 10], 20), 2);
        assert_eq!(Solution::min_patches(vec![1, 2, 2], 5), 0);
    }
}

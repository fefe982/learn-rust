// https://leetcode.com/problems/create-sorted-array-through-instructions/
// 1649. Create Sorted Array through Instructions
pub struct Solution;
const N: usize = 131072;
impl Solution {
    fn add(cnt: &mut Vec<i32>, v: i32) {
        let mut n = N + v as usize;
        while n > 0 {
            cnt[n] += 1;
            n /= 2;
        }
    }
    fn count(cnt: &Vec<i32>, i: usize, si: i32, ei: i32, from: i32, to: i32) -> i32 {
        if to < si || from > ei {
            return 0;
        }
        if from <= si && ei <= to {
            return cnt[i];
        }
        let mid = (si + ei) / 2;
        Self::count(cnt, i * 2, si, mid, from, to) + Self::count(cnt, i * 2 + 1, mid + 1, ei, from, to)
    }
    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        let mut cnt = vec![0; N * 2];
        let mut ans = 0;
        for i in instructions {
            Self::add(&mut cnt, i);
            ans += Self::count(&cnt, 1, 0, N as i32 - 1, 0, i - 1).min(Self::count(
                &cnt,
                1,
                0,
                N as i32 - 1,
                i + 1,
                N as i32 - 1,
            )) as i64;
        }
        (ans % 1000000007) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_sorted_array() {
        assert_eq!(Solution::create_sorted_array(vec![1, 5, 6, 2]), 1);
        assert_eq!(Solution::create_sorted_array(vec![1, 2, 3, 6, 5, 4]), 3);
        assert_eq!(Solution::create_sorted_array(vec![1, 3, 3, 3, 2, 4, 2, 1, 2]), 4);
    }
}

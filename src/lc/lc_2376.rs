// https://leetcode.com/problems/count-special-integers/
// 2376. Count Special Integers
pub struct Solution;
impl Solution {
    pub fn count_special_numbers(n: i32) -> i32 {
        let mut cnt = vec![
            0,
            9,
            9 * 9,
            9 * 9 * 8,
            9 * 9 * 8 * 7,
            9 * 9 * 8 * 7 * 6,
            9 * 9 * 8 * 7 * 6 * 5,
            9 * 9 * 8 * 7 * 6 * 5 * 4,
            9 * 9 * 8 * 7 * 6 * 5 * 4 * 3,
            9 * 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2,
        ];
        let max = vec![0, 9, 98, 987, 9876, 98765, 987654, 9876543, 98765432, 987654321];
        let offset = vec![
            1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
        ];
        for i in 1..cnt.len() {
            cnt[i] = cnt[i] + cnt[i - 1];
        }
        let mut n = n + 1;
        let mut pos = max.partition_point(|&x| x < n);
        let mut ans = cnt[pos - 1];
        if n < offset[pos - 1] {
            pos -= 1;
        }
        let mut mul = 9;
        let mut partial = 1;
        for _ in 1..pos {
            partial *= mul;
            mul -= 1;
        }
        let mut used = vec![false; 10];
        mul = 9;
        let mut start = 1;
        for i in (0..pos).rev() {
            let d = n / offset[i];
            let mut unused = 0;
            for j in start..d as usize {
                if !used[j] {
                    unused += 1;
                }
            }
            start = 0;
            ans += unused * partial;
            if used[d as usize] {
                break;
            }
            used[d as usize] = true;
            partial /= mul;
            mul -= 1;
            n %= offset[i];
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_special_numbers() {
        assert_eq!(Solution::count_special_numbers(97), 89);
        assert_eq!(Solution::count_special_numbers(20), 19);
        assert_eq!(Solution::count_special_numbers(5), 5);
        assert_eq!(Solution::count_special_numbers(135), 110);
    }
}

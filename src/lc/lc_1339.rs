// https://leetcode.cn/problems/count-largest-group/
// 1339. Maximum Product of Splitted Binary Tree
pub struct Solution;
impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut cnt = vec![0; 37];
        let mut s = vec![0; 10001];
        for i in 1..=n as usize {
            let si = s[i / 10] + i % 10;
            s[i] = si;
            cnt[si as usize] += 1;
        }
        let mut ans = 0;
        let mut max = 0;
        for i in 1..=36 {
            if cnt[i] > max {
                max = cnt[i];
                ans = 1;
            } else if cnt[i] == max {
                ans += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_largest_group() {
        assert_eq!(Solution::count_largest_group(13), 4);
        assert_eq!(Solution::count_largest_group(2), 2);
    }
}

// https://leetcode.com/problems/find-palindrome-with-fixed-length/
// 2217. Find Palindrome With Fixed Length
pub struct Solution;
impl Solution {
    pub fn kth_palindrome(queries: Vec<i32>, int_length: i32) -> Vec<i64> {
        let mut ans = Vec::with_capacity(queries.len());
        let half = (int_length + 1) / 2;
        let start = 10i64.pow(half as u32 - 1);
        for q in queries {
            let num = start + q as i64 - 1;
            if num >= start * 10 {
                ans.push(-1);
            } else {
                let mut a = num;
                let mut b = a;
                if int_length % 2 == 1 {
                    b /= 10;
                }
                while b > 0 {
                    a = a * 10 + b % 10;
                    b /= 10;
                }
                ans.push(a);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn kth_palindrome() {
        assert_eq!(
            Solution::kth_palindrome(vec![1, 2, 3, 4, 5, 90], 3),
            vec![101, 111, 121, 131, 141, 999]
        );
        assert_eq!(Solution::kth_palindrome(vec![2, 4, 6], 4), vec![1111, 1331, 1551]);
    }
}

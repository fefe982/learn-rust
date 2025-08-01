// https://leetcode.com/problems/count-distinct-numbers-on-board/
// 2549. Count Distinct Numbers on Board
pub struct Solution;
impl Solution {
    pub fn distinct_integers(n: i32) -> i32 {
        if n <= 2 {
            return 1;
        }
        let mut v = vec![false; n as usize + 1];
        v[n as usize] = true;
        let mut ans = 0;
        for i in (2..=n).rev() {
            if v[i as usize] {
                v[i as usize - 1] = true;
                ans += 1;
                let mut j = 2;
                let m = n - 1;
                while j * j <= m {
                    if m % j == 0 {
                        v[j as usize] = true;
                        v[(m / j) as usize] = true;
                    }
                    j += 1;
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_distinct_integers() {
        assert_eq!(Solution::distinct_integers(5), 4);
        assert_eq!(Solution::distinct_integers(3), 2);
    }
}

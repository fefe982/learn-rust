// https://leetcode.com/problems/unique-3-digit-even-numbers/
// 3483. Unique 3-Digit Even Numbers
pub struct Solution;
impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let mut cnt = vec![0; 10];
        for &d in &digits {
            cnt[d as usize] += 1;
        }
        let mut nz = 0;
        let mut nd = 0;
        for i in 1..10 {
            if cnt[i] > 0 {
                nz += 1;
                if cnt[i] > 1 {
                    nd += 1;
                }
            }
        }
        let mut ans = 0;
        for i in 1..5 {
            if cnt[i * 2] >= 3 {
                ans += 1;
            }
            if cnt[i * 2] >= 2 {
                ans += 2 * (nz - 1);
                if cnt[0] != 0 {
                    ans += 1;
                }
            }
            if cnt[i * 2] >= 1 {
                ans += (nz - 1) * (nz - 2);
                if cnt[0] != 0 {
                    ans += nz - 1;
                }
                if cnt[i * 2] >= 2 {
                    ans += nd - 1;
                } else {
                    ans += nd;
                }
            }
        }
        if cnt[0] >= 2 {
            ans += nz;
        }
        if cnt[0] >= 1 {
            ans += nd;
            ans += nz * (nz - 1);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn total_numbers() {
        assert_eq!(Solution::total_numbers(vec![1, 2, 3, 4]), 12);
        assert_eq!(Solution::total_numbers(vec![0, 2, 2]), 2);
        assert_eq!(Solution::total_numbers(vec![6, 6, 6]), 1);
        assert_eq!(Solution::total_numbers(vec![1, 3, 5]), 0);
    }
}

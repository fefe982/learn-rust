// https://leetcode.com/problems/count-stepping-numbers-in-range/
// 2801. Count Stepping Numbers in Range
pub struct Solution;
const MOD: i64 = 1_0000_0000_7;
impl Solution {
    fn count(n: String, inlcusive: bool) -> i64 {
        let n = n.as_bytes();
        let mut cnt = vec![0; 10];
        for i in 1..(n[0] - b'0') as usize {
            cnt[i] = 1;
        }
        let mut bound = true;
        for i in 1..n.len() {
            let mut ncnt = vec![0; 10];
            for j in 1..9 {
                ncnt[j] = (cnt[j - 1] + cnt[j + 1] + 1) % MOD;
            }
            ncnt[0] = cnt[1];
            ncnt[9] = cnt[8] + 1;
            if bound {
                if n[i - 1] > b'0' {
                    if n[i] > n[i - 1] - 1 {
                        ncnt[(n[i - 1] - b'0') as usize - 1] += 1;
                    }
                }
                if n[i - 1] < b'9' {
                    if n[i] > n[i - 1] + 1 {
                        ncnt[(n[i - 1] - b'0') as usize + 1] += 1;
                    }
                }
                if n[i] != n[i - 1] + 1 && n[i] != n[i - 1] - 1 {
                    bound = false;
                }
            }
            cnt = ncnt;
        }
        let mut ans = if inlcusive && bound { 1 } else { 0 };
        for i in 0..10 {
            ans = (ans + cnt[i]) % MOD;
        }
        ans
    }
    pub fn count_stepping_numbers(low: String, high: String) -> i32 {
        return ((Self::count(high, true) - Self::count(low, false) + MOD) % MOD) as i32;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_stepping_numbers() {
        assert_eq!(Solution::count_stepping_numbers("1".to_string(), "11".to_string()), 10);
        assert_eq!(Solution::count_stepping_numbers("90".to_string(), "101".to_string()), 2);
    }
}

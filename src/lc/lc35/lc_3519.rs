// https://leetcode.com/problems/count-numbers-with-non-decreasing-digits/
// 3519. Count Numbers with Non-Decreasing Digits
pub struct Solution;
impl Solution {
    fn to_vec(n: String, b: i32) -> Vec<i32> {
        let mut n = n.as_bytes().iter().map(|c| (c - b'0') as i32).collect::<Vec<_>>();
        let mut v = vec![];
        let mut s = 0;
        while s < n.len() {
            let mut r = 0;
            for i in s..n.len() {
                let d = r * 10 + n[i];
                n[i] = d / b;
                r = d % b;
            }
            v.push(r);
            while s < n.len() && n[s] == 0 {
                s += 1;
            }
        }
        v
    }

    pub fn count_numbers(l: String, r: String, b: i32) -> i32 {
        let mut l = Self::to_vec(l, b);
        let mut r = Self::to_vec(r, b);
        l.resize(r.len(), 0);
        for i in (0..l.len() - 1).rev() {
            if l[i] < l[i + 1] {
                for j in (0..=i).rev() {
                    l[j] = l[i + 1];
                }
                break;
            }
        }
        for i in (0..r.len() - 1).rev() {
            if r[i] < r[i + 1] {
                let mut j = i + 1;
                r[j] -= 1;
                while j + 1 < r.len() && r[j] < r[j + 1] {
                    j += 1;
                    r[j] -= 1;
                }
                for k in 0..j {
                    r[k] = b - 1;
                }
                break;
            }
        }
        let mut cnt = vec![vec![0i64; b as usize + 1]; r.len() + 1];
        for i in 0..=b as usize {
            cnt[0][i] = 1;
        }
        for i in 1..=r.len() {
            cnt[i][b as usize] = 0;
            for j in (0..b as usize).rev() {
                cnt[i][j] = cnt[i - 1][j] + cnt[i][j + 1];
            }
        }
        let mut s = r.len() - 1;
        while s > 0 && l[s] == r[s] {
            s -= 1;
        }
        if l[s] == r[s] {
            return 1;
        }
        let mut ans = cnt[s + 1][l[s] as usize + 1] - cnt[s + 1][r[s] as usize];
        for i in (0..s).rev() {
            ans += cnt[i + 1][l[i] as usize + 1];
            ans += cnt[i + 1][r[i + 1] as usize] - cnt[i + 1][r[i] as usize];
        }
        ((ans + 2) % 1000000007) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_numbers() {
        assert_eq!(Solution::count_numbers("23".to_string(), "28".to_string(), 8), 3);
        assert_eq!(Solution::count_numbers("2".to_string(), "7".to_string(), 2), 2);
    }
}

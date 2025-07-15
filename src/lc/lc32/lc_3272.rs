// https://leetcode.com/problems/find-the-count-of-good-integers/
// 3272. Find the Count of Good Integers
pub struct Solution;
impl Solution {
    pub fn count_good_integers(n: i32, k: i32) -> i64 {
        if n == 1 {
            return (9 / k) as i64;
        }
        let n = n as usize;
        let mut cnk = vec![vec![]; n + 1];
        cnk[0].push(1i64);
        for i in 1..=n {
            cnk[i].push(1);
            for j in 1..i {
                let ck = cnk[i - 1][j - 1] + cnk[i - 1][j];
                cnk[i].push(ck);
            }
            cnk[i].push(1);
        }
        let h2 = n / 2;
        let mut p = vec![1i64; 10];
        for i in 1..10 {
            p[i] = p[i - 1] * 10;
        }
        let mut left = p[h2 - 1];
        let mut leftp = left * 10;
        let mut mid = 0;
        if n % 2 == 1 {
            mid = leftp;
            leftp = mid * 10;
        }
        left *= leftp;
        let mut vr = vec![0; h2];
        let mut right = 1;
        vr[h2 - 1] = 1;
        let mut ans = 0;
        let mut dmid = 0;
        let k = k as i64;
        let mut set = std::collections::HashSet::new();
        while right > 0 {
            let num = left + right + mid * dmid;
            if num % k == 0 {
                let mut d = vec![0; 10];
                let mut key = 0;
                for &r in &vr {
                    d[r as usize] += 2;
                    key += p[r as usize] * 2;
                }
                if mid > 0 {
                    d[dmid as usize] += 1;
                    key += p[dmid as usize];
                }
                if set.insert(key) {
                    let mut cnt = cnk[n - 1][d[0]];
                    let mut rem = n - d[0];
                    for i in 1..10 {
                        if d[i] > 0 {
                            cnt *= cnk[rem][d[i]];
                        }
                        rem -= d[i];
                    }
                    ans += cnt;
                }
            }
            if n % 2 == 1 {
                if dmid < 9 {
                    dmid += 1;
                    continue;
                } else {
                    dmid = 0;
                }
            }
            left += leftp;
            for i in 0..h2 {
                if vr[i] < 9 {
                    vr[i] += 1;
                    right += p[h2 - i - 1];
                    break;
                } else {
                    vr[i] = 0;
                    right -= p[h2 - i - 1] * 9;
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
    fn count_good_integers() {
        assert_eq!(Solution::count_good_integers(3, 5), 27);
        assert_eq!(Solution::count_good_integers(1, 4), 2);
        assert_eq!(Solution::count_good_integers(5, 6), 2468);
    }
}

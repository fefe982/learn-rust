// https://leetcode.com/problems/count-number-of-balanced-permutations/
// 3343. Count Number of Balanced Permutations
pub struct Solution;
const MOD: i64 = 1_000_000_007;
impl Solution {
    fn search(
        num: usize,
        odd: i32,
        even: i32,
        oleft: i32,
        eleft: i32,
        cache: &mut Vec<Vec<Vec<i64>>>,
        rf: &Vec<i64>,
        cnt: &[i32; 10],
    ) -> i64 {
        if num == 10 {
            if odd == 0 && oleft == 0 {
                return 1;
            } else {
                return 0;
            }
        }
        if odd < 0 || even < 0 || oleft < 0 || eleft < 0 {
            return 0;
        }
        if cache[num][odd as usize][oleft as usize] != -1 {
            return cache[num][odd as usize][oleft as usize];
        }
        let mut res = 0;
        for i in 0.max(cnt[num] - even)..=cnt[num].min(odd) {
            res += Self::search(
                num + 1,
                odd - i,
                even - (cnt[num] - i),
                oleft - i * num as i32,
                eleft - (cnt[num] - i) * num as i32,
                cache,
                rf,
                cnt,
            ) * rf[i as usize]
                % MOD
                * rf[(cnt[num] - i) as usize]
                % MOD;
        }
        res %= MOD;
        cache[num][odd as usize][oleft as usize] = res;
        res
    }
    pub fn count_balanced_permutations(num: String) -> i32 {
        let num = num.as_bytes();
        let mut cnt = [0; 10];
        let mut sum = 0;
        for &n in num {
            cnt[(n - b'0') as usize] += 1;
            sum += (n - b'0') as i32;
        }
        if sum % 2 == 1 {
            return 0;
        }
        let len = num.len();
        let div = |mut a: i64, mut b: i64| -> i64 {
            while a % b != 0 {
                let n = MOD / b;
                a = a * (n + 1) % MOD;
                b = b * (n + 1) % MOD;
            }
            a / b
        };
        let hlen = (len + 1) / 2;
        let mut f = vec![1; hlen + 1];
        let mut rf = vec![1; hlen + 1];
        for i in 2..=hlen {
            f[i] = f[i - 1] * i as i64 % MOD;
            rf[i] = div(rf[i - 1], i as i64);
        }
        (f[hlen] * f[len / 2] % MOD
            * Self::search(
                0,
                hlen as i32,
                (len - hlen) as i32,
                sum / 2,
                sum / 2,
                &mut vec![vec![vec![-1; sum as usize / 2 + 1]; hlen + 1]; 10],
                &rf,
                &cnt,
            )
            % MOD) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_balanced_permutations() {
        assert_eq!(Solution::count_balanced_permutations("264666".to_string()), 0);
        assert_eq!(Solution::count_balanced_permutations("123".to_string()), 2);
        assert_eq!(Solution::count_balanced_permutations("112".to_string()), 1);
        assert_eq!(Solution::count_balanced_permutations("12345".to_string()), 0);
    }
}

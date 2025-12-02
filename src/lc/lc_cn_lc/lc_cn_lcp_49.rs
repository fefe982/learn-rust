// https://leetcode.cn/problems/K8GULz/
// LCP 49. 环形闯关游戏
pub struct Solution;
impl Solution {
    pub fn ring_game(challenge: Vec<i64>) -> i64 {
        let n = challenge.len();
        let mut sl = vec![0; n * 3];
        let mut il = vec![0; n * 3];
        for i in 0..n * 3 {
            sl[i] = challenge[i % n];
            let mut j = i;
            while j > 0 && challenge[(j - 1) % n] <= sl[i] {
                sl[i] |= sl[j - 1];
                j = il[j - 1];
            }
            il[i] = j;
        }
        let mut rl = vec![0; n * 3];
        let mut ir = vec![0; n * 3];
        for i in (0..n * 3).rev() {
            rl[i] = challenge[i % n];
            let mut j = i;
            while j < n * 3 - 1 && challenge[(j + 1) % n] <= rl[i] {
                rl[i] |= rl[j + 1];
                j = ir[j + 1];
            }
            ir[i] = j;
        }
        let max = challenge.iter().max().unwrap();
        let mut bit = 1i64 << (i64::BITS - max.leading_zeros() - 1);
        let mut res = bit;
        loop {
            bit >>= 1;
            if bit == 0 {
                break;
            }
            let mut i = n;
            let mut good = false;
            let chk = res | (bit - 1);
            'check: while i < n + n {
                if challenge[i - n] > chk {
                    i += 1;
                    continue;
                }
                let mut s = chk | sl[i] | rl[i];
                let mut l = il[i];
                let mut r = ir[i];
                let mut len = 1;
                loop {
                    if l > 0 && s >= challenge[(l - 1) % n] {
                        s |= sl[l - 1];
                        l = il[l - 1];
                    }
                    if r < n * 3 - 1 && s >= challenge[(r + 1) % n] {
                        s |= rl[r + 1];
                        r = ir[r + 1];
                    }
                    let nl = r - l + 1;
                    if nl >= n {
                        good = true;
                        break 'check;
                    }
                    if nl == len {
                        break;
                    }
                    len = nl;
                }
                i = r + 1;
            }
            if !good {
                res |= bit;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ring_game() {
        assert_eq!(Solution::ring_game(vec![63, 1, 4, 57, 57, 59, 63, 1, 5]), 56);
        assert_eq!(Solution::ring_game(vec![5, 4, 6, 2, 7]), 4);
        assert_eq!(Solution::ring_game(vec![12, 7, 11, 3, 9]), 8);
        assert_eq!(Solution::ring_game(vec![1, 1, 1]), 1);
    }
}

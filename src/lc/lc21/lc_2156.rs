// https://leetcode.com/problems/find-substring-with-given-hash-value/
// 2156. Find Substring With Given Hash Value
pub struct Solution;
impl Solution {
    fn egcd(mut a: i64, mut b: i64) -> (i64, (i64, i64)) {
        let mut ia = (1, 0);
        let mut ib = (0, 1);
        loop {
            if a % b == 0 {
                return (b, ib);
            }
            let q = a / b;
            ia = (ia.0 - ib.0 * q, ia.1 - ib.1 * q);
            a = a % b;
            if b % a == 0 {
                return (a, ia);
            }
            let q = b / a;
            ib = (ib.0 - ia.0 * q, ib.1 - ia.1 * q);
            b = b % a;
        }
    }
    pub fn sub_str_hash(s: String, power: i32, modulo: i32, k: i32, hash_value: i32) -> String {
        let power = power as i64;
        let modulo = modulo as i64;
        let hash = hash_value as i64;
        let k = k as usize;
        let (gcd, (inv_p, _)) = Self::egcd(power, modulo);
        let s = s.as_bytes();
        let k = k as usize;
        if gcd == 1 {
            let inv_p = (inv_p % modulo + modulo) % modulo;
            let mut i = 0;
            let mut h = 0;
            let mut p = 1;
            for j in 0..k {
                h = ((s[j] - b'a' + 1) as i64 * p + h) % modulo;
                p = (p * power) % modulo;
            }
            while h != hash {
                h = (h - (s[i] - b'a' + 1) as i64 + (s[i + k] - b'a' + 1) as i64 * p + modulo) % modulo;
                h = (h * inv_p) % modulo;
                i += 1;
            }
            String::from_utf8(s[i..i + k].to_vec()).unwrap()
        } else {
            let mut r = 0;
            let mut i = s.len() - k;
            let mut h = 0;
            let mut p = 1;
            for j in 0..k {
                h = ((s[i + j] - b'a' + 1) as i64 * p + h) % modulo;
                p = (p * power) % modulo;
            }
            if h == hash {
                r = i;
            }
            while i > 0 {
                i -= 1;
                h = (h * power - (s[i + k] - b'a' + 1) as i64 * p + (s[i] - b'a' + 1) as i64) % modulo;
                h = (h + modulo) % modulo;
                if h == hash {
                    r = i;
                }
            }
            String::from_utf8(s[r..r + k].to_vec()).unwrap()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sub_str_hash() {
        assert_eq!(
            Solution::sub_str_hash(
                "dlojuxgftvpqpsknfgkejydsxgcgyroavsefjrejytcgflrnnxxsxowqbteycujnrbaokjibq".to_string(),
                8,
                54,
                30,
                16
            ),
            "fjrejytcgflrnnxxsxowqbteycujnr"
        );
        assert_eq!(Solution::sub_str_hash("leetcode".to_string(), 7, 20, 2, 0), "ee");
        assert_eq!(Solution::sub_str_hash("fbxzaad".to_string(), 31, 100, 3, 32), "fbx");
    }
}

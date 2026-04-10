// https://leetcode.com/problems/incremental-memory-leak/
// 1860. Incremental Memory Leak
pub struct Solution;
impl Solution {
    pub fn mem_leak(memory1: i32, memory2: i32) -> Vec<i32> {
        fn tri(n: i128) -> i128 {
            n * (n + 1) / 2
        }

        fn isqrt(n: i128) -> i128 {
            let mut x = (n as f64).sqrt() as i128;
            while (x + 1) * (x + 1) <= n {
                x += 1;
            }
            while x * x > n {
                x -= 1;
            }
            x
        }

        fn max_prefix_by_diff(diff: i128) -> i128 {
            let mut k = (isqrt(8 * diff + 1) - 1) / 2;
            while tri(k + 1) <= diff {
                k += 1;
            }
            while tri(k) > diff {
                k -= 1;
            }
            k
        }

        fn max_r(total: i128, base: i128) -> i128 {
            let mut r = (isqrt(base * base + 4 * total) - base) / 2;
            while (r + 1) * (base + r + 1) <= total {
                r += 1;
            }
            while r * (base + r) > total {
                r -= 1;
            }
            r
        }

        let mut m1 = memory1 as i128;
        let mut m2 = memory2 as i128;
        let diff = (m1 - m2).abs();
        let k = max_prefix_by_diff(diff);

        if m1 >= m2 {
            m1 -= tri(k);
        } else {
            m2 -= tri(k);
        }

        let mut t = k + 1;

        let start_on_m1 = m1 >= m2;
        let mut a = if start_on_m1 { m1 } else { m2 };
        let mut b = if start_on_m1 { m2 } else { m1 };

        let pairs = max_r(a, t - 1).min(max_r(b, t));
        a -= pairs * (t + pairs - 1);
        b -= pairs * (t + pairs);
        t += 2 * pairs;

        if a >= t {
            a -= t;
            t += 1;
            if b >= t {
                b -= t;
                t += 1;
            }
        }

        if start_on_m1 {
            vec![t as i32, a as i32, b as i32]
        } else {
            vec![t as i32, b as i32, a as i32]
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mem_leak() {
        assert_eq!(Solution::mem_leak(2, 2), vec![3, 1, 0]);
        assert_eq!(Solution::mem_leak(8, 11), vec![6, 0, 4]);
    }
}

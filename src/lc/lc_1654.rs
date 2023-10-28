// https://leetcode.com/problems/minimum-jumps-to-reach-home/
// 1654. Minimum Jumps to Reach Home
pub struct Solution;
impl Solution {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while a % b != 0 {
            a = a % b;
            if b % a == 0 {
                return a;
            }
            b = b % a;
        }
        b
    }
    pub fn minimum_jumps(forbidden: Vec<i32>, mut a: i32, mut b: i32, mut x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let g = Self::gcd(a, b);
        if x % g != 0 {
            return -1;
        }
        a /= g;
        b /= g;
        x /= g;
        let mut m = 0;
        let mut f: std::collections::HashSet<(i32, i32)> = forbidden
            .into_iter()
            .filter_map(|x| {
                if x % g == 0 {
                    let xg = x / g;
                    m = m.max(xg);
                    Some(vec![(xg, 1), (xg, -1)])
                } else {
                    None
                }
            })
            .flatten()
            .collect();
        m = (m + a).max(x) + b;
        let mut q = std::collections::VecDeque::<(i32, i32, i32)>::new();
        q.push_back((0, 0, 1));
        while let Some((p, s, d)) = q.pop_front() {
            if f.contains(&(p, d)) {
                continue;
            }
            f.insert((p, d));
            let fp = p + a;
            if fp == x {
                return s + 1;
            }
            if fp <= m && !f.contains(&(fp, 1)) {
                q.push_back((fp, s + 1, 1));
            }
            if d == 1 {
                let bp = p - b;
                if bp == x {
                    return s + 1;
                }
                if bp > 0 && !f.contains(&(bp, -1)) {
                    q.push_back((bp, s + 1, -1));
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_jumps() {
        assert_eq!(
            Solution::minimum_jumps(
                vec![
                    162, 118, 178, 152, 167, 100, 40, 74, 199, 186, 26, 73, 200, 127, 30, 124, 193,
                    84, 184, 36, 103, 149, 153, 9, 54, 154, 133, 95, 45, 198, 79, 157, 64, 122, 59,
                    71, 48, 177, 82, 35, 14, 176, 16, 108, 111, 6, 168, 31, 134, 164, 136, 72, 98
                ],
                29,
                98,
                80
            ),
            121
        );
        assert_eq!(Solution::minimum_jumps(vec![14, 4, 18, 1, 15], 3, 15, 9), 3);
        assert_eq!(
            Solution::minimum_jumps(vec![8, 3, 16, 6, 12, 20], 15, 13, 11),
            -1
        );
        assert_eq!(
            Solution::minimum_jumps(vec![1, 6, 2, 14, 5, 17, 4], 16, 9, 7),
            2
        );
    }
}

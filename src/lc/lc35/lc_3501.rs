// https://leetcode.com/problems/maximize-active-section-with-trade-ii/
// 3501. Maximize Active Sections With Trade-ii
pub struct Solution;
impl Solution {
    pub fn max_active_sections_after_trade(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let s = s.as_bytes();
        let mut vp = vec![(-1, -1)];
        let mut i = 0;
        let mut cnt = 0;
        for j in 0..s.len() {
            if j == s.len() - 1 || (j < s.len() - 1 && s[j] != s[j + 1]) {
                if s[j] == b'1' {
                    cnt += (j - i + 1) as i32;
                } else {
                    vp.push((i as i32, j as i32 + 1));
                }
                i = j + 1;
            }
        }
        vp.push((s.len() as i32 + 1, s.len() as i32 + 1));
        let tlen = 2 << (usize::BITS - (vp.len() - 1).leading_zeros());
        let mut v = vec![0; tlen];
        for i in 1..vp.len() - 2 {
            v[tlen / 2 - 1 + i] = vp[i].1 - vp[i].0 + vp[i + 1].1 - vp[i + 1].0;
        }
        for i in (0..tlen / 2 - 1).rev() {
            v[i] = v[2 * i + 1].max(v[2 * i + 2]);
        }
        fn query(v: &Vec<i32>, iv: usize, l: usize, r: usize, ql: usize, qr: usize) -> i32 {
            if ql == qr || l >= qr || r <= ql {
                return 0;
            } else if l >= ql && r <= qr {
                return v[iv];
            } else {
                let mid = (l + r) / 2;
                return query(v, iv * 2 + 1, l, mid, ql, qr).max(query(v, iv * 2 + 2, mid, r, ql, qr));
            }
        }
        let mut ans = vec![cnt; queries.len()];
        let cntz = |l: i32, r: i32| {
            if l > 0 && r > 0 {
                l + r
            } else {
                0
            }
        };
        for (i, q) in queries.iter().enumerate() {
            let l = q[0];
            let r = q[1] + 1;
            let il = vp.partition_point(|&(s, _)| l >= s) - 1;
            let ir = vp.partition_point(|&(_, e)| r >= e);
            if il == ir {
                continue;
            } else if il + 1 == ir {
                ans[i] += cntz(vp[il].1 - l, r - vp[ir].0);
            } else {
                ans[i] += cntz(vp[il].1 - l, vp[il + 1].1 - vp[il + 1].0)
                    .max(query(&v, 0, 0, tlen / 2, il + 1, ir - 1))
                    .max(cntz(vp[ir - 1].1 - vp[ir - 1].0, r - vp[ir].0));
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_active_sections_after_trade() {
        assert_eq!(
            Solution::max_active_sections_after_trade("01".to_string(), vec_vec![[0, 1]]),
            [1]
        );
        assert_eq!(
            Solution::max_active_sections_after_trade("0100".to_string(), vec_vec![[0, 3], [0, 2], [1, 3], [2, 3]]),
            [4, 3, 1, 1]
        );
        assert_eq!(
            Solution::max_active_sections_after_trade("1000100".to_string(), vec_vec![[1, 5], [0, 6], [0, 4]]),
            [6, 7, 2]
        );
        assert_eq!(
            Solution::max_active_sections_after_trade("01010".to_string(), vec_vec![[0, 3], [1, 4], [1, 3]]),
            [4, 4, 2]
        );
    }
}

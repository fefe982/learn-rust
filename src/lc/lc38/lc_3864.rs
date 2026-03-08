// https://leetcode.com/problems/minimum-cost-to-partition-a-binary-string/
// 3864. Minimum Cost to Partition a Binary String
pub struct Solution;
impl Solution {
    fn min_cost_seg(cnt: &Vec<i32>, enc_cost: i64, flat_cost: i64, i: usize, j: usize) -> i64 {
        if cnt[j] - cnt[i] == 0 {
            return flat_cost;
        }
        let mut cost = (j - i) as i64 * (cnt[j] - cnt[i]) as i64 * enc_cost;
        if (j - i) % 2 == 0 {
            let mid = (i + j) / 2;
            cost = cost.min(
                Self::min_cost_seg(cnt, enc_cost, flat_cost, i, mid)
                    + Self::min_cost_seg(cnt, enc_cost, flat_cost, mid, j),
            );
        }
        cost
    }
    pub fn min_cost(s: String, enc_cost: i32, flat_cost: i32) -> i64 {
        let s = s.as_bytes();
        let mut cnt = vec![0; s.len() + 1];
        for i in 0..s.len() {
            cnt[i + 1] = cnt[i] + if s[i] == b'1' { 1 } else { 0 };
        }
        Self::min_cost_seg(&cnt, enc_cost as i64, flat_cost as i64, 0, s.len())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_cost() {
        assert_eq!(Solution::min_cost("1010".to_string(), 2, 1), 6);
        assert_eq!(Solution::min_cost("1010".to_string(), 3, 10), 12);
        assert_eq!(Solution::min_cost("00".to_string(), 1, 2), 2);
    }
}

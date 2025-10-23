// https://leetcode.com/problems/minimum-cost-good-caption/
// 3441. Minimum Cost to Cut a Stick
pub struct Solution;
impl Solution {
    pub fn min_cost_good_caption(caption: String) -> String {
        let cap = caption.chars().map(|b| b as i32).collect::<Vec<_>>();
        if cap.len() < 3 {
            return "".to_string();
        }
        let len = cap.len();
        let mut dp = vec![(0, 0, usize::MAX); len + 1];
        dp[len - 1].0 = i32::MAX - 52;
        dp[len - 2].0 = i32::MAX - 52;
        for i in (0..len - 2).rev() {
            let mut min = i32::MAX;
            let mut minc = 0;
            let mut minnc = 0;
            let mut minj = 0;
            for j in 3..6 {
                if i + j > len {
                    break;
                }
                let mut v = cap[i..i + j].iter().cloned().collect::<Vec<_>>();
                v.sort();
                let c = v[(j - 1) / 2];
                let sum = v.iter().fold(0, |sum, &x| sum + (x - c).abs());
                if sum + dp[i + j].0 < min
                    || sum + dp[i + j].0 == min && c < minc
                    || sum + dp[i + j].0 == min && c == minc && c < minnc
                    || sum + dp[i + j].0 == min && c == minc && c == minnc && dp[i + j].1 < minnc
                {
                    min = sum + dp[i + j].0;
                    minc = c;
                    minnc = dp[i + j].1;
                    minj = j;
                }
            }
            dp[i].0 = min;
            dp[i].1 = minc;
            dp[i].2 = minj;
        }
        let mut s = String::with_capacity(len);
        let mut i = 0;
        while i < len {
            let j = dp[i].2;
            let c = dp[i].1;
            for _ in 0..j {
                s.push(c as u8 as char);
            }
            i += j;
        }
        s
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_cost_good_caption() {
        assert_eq!(
            Solution::min_cost_good_caption("dlqlawbgd".to_string()),
            "llllddddd".to_string()
        );
        assert_eq!(Solution::min_cost_good_caption("cdcd".to_string()), "cccc".to_string());
        assert_eq!(Solution::min_cost_good_caption("aca".to_string()), "aaa".to_string());
        assert_eq!(Solution::min_cost_good_caption("bc".to_string()), "".to_string());
    }
}

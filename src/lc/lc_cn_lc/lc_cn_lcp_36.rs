// https://leetcode.cn/problems/Up5XYM/
// LCP 36. 最多牌组数
pub struct Solution;
impl Solution {
    pub fn max_group_number(tiles: Vec<i32>) -> i32 {
        let mut cnt = std::collections::BTreeMap::<i32, usize>::new();
        for t in tiles {
            *cnt.entry(t).or_insert(0) += 1;
        }
        let mut dp = vec![vec![-1; 5]; 5];
        let mut last = -1;
        dp[0][0] = 0;
        for (i, v) in cnt.into_iter() {
            if i != last + 1 {
                let dp00 = dp[0][0];
                dp.iter_mut().for_each(|x| x.fill(-1));
                dp[0][0] = dp00;
            }
            let mut ndp = vec![vec![-1; 5]; 5];
            for j in 0..5 {
                for k in 0..5 {
                    if dp[j][k] == -1 {
                        continue;
                    }
                    for s in 0..=j.min(k).min(v) {
                        for l in 0..=(v - s).min(4) {
                            ndp[k - s][l] = ndp[k - s][l].max(dp[j][k] + (s + (v - s - l) / 3) as i32);
                        }
                    }
                }
            }
            dp = ndp;
            last = i;
        }
        dp[0][0] as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::max_group_number(vec![2, 2, 2, 3, 3, 3, 4, 4, 4, 4, 7, 7, 7, 8, 9, 9, 10, 10]),
            5
        );
        assert_eq!(Solution::max_group_number(vec![2, 2, 2, 3, 4]), 1);
        assert_eq!(Solution::max_group_number(vec![2, 2, 2, 3, 4, 1, 3]), 2);
    }
}

// https://leetcode.com/problems/maximum-nesting-depth-of-two-valid-parentheses-strings/
// 1111. Maximum Nesting Depth of Two Valid Parentheses Strings
pub struct Solution;
impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let n = seq.len();
        let mut res = Vec::with_capacity(n);
        let mut cdep = 0;
        let mut mdep = 0;
        for c in seq.chars() {
            if c == '(' {
                res.push(cdep);
                mdep = mdep.max(cdep);
                cdep += 1;
            } else {
                cdep -= 1;
                res.push(cdep);
            }
        }
        for i in 0..n {
            if res[i] <= mdep / 2 {
                res[i] = 0;
            } else {
                res[i] = 1;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(seq: &str, expect: Vec<i32>) {
        let res = Solution::max_depth_after_split(seq.to_string());
        let n = seq.len();
        assert_eq!(res.len(), n);
        fn depth(seq: &str, partition: &Vec<i32>) -> i32 {
            let mut cdep0 = 0;
            let mut cdep1 = 0;
            let mut mdep0 = 0;
            let mut mdep1 = 0;
            for (c, &i) in seq.chars().zip(partition.iter()) {
                assert!(i == 0 || i == 1);
                if c == '(' {
                    if i == 0 {
                        cdep0 += 1;
                        mdep0 = mdep0.max(cdep0);
                    } else {
                        cdep1 += 1;
                        mdep1 = mdep1.max(cdep1);
                    }
                } else {
                    if i == 0 {
                        cdep0 -= 1;
                        assert!(cdep0 >= 0);
                    } else {
                        cdep1 -= 1;
                        assert!(cdep1 >= 0);
                    }
                }
            }
            assert_eq!(cdep0, 0);
            assert_eq!(cdep1, 0);
            mdep0.min(mdep1)
        }
        let expect_depth = depth(seq, &expect);
        let res_depth = depth(seq, &res);
        assert_eq!(res_depth, expect_depth);
    }
    #[test]
    fn max_depth_after_split() {
        check("(((()))((())))", vec![0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1]);
        check("(()())", vec![0, 1, 1, 1, 1, 0]);
        check("()(())()", vec![0, 0, 0, 1, 1, 0, 1, 1]);
    }
}

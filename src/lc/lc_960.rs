// https://leetcode.com/problems/delete-columns-to-make-sorted-iii/
// 960. Delete Columns to Make Sorted III
pub struct Solution;
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let strs = strs
            .into_iter()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut ans = strs[0].len() as i32 - 1;
        let mut min_d = vec![0; strs[0].len()];
        for i in 1..strs[0].len() {
            min_d[i] = i as i32;
            for j in 0..i {
                let mut good = true;
                for k in 0..strs.len() {
                    if strs[k][j] > strs[k][i] {
                        good = false;
                        break;
                    }
                }
                if good {
                    min_d[i] = min_d[i].min(min_d[j] + (i - j) as i32 - 1);
                }
            }
            ans = ans.min(min_d[i] + (strs[0].len() - i - 1) as i32);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_deletion_size() {
        assert_eq!(Solution::min_deletion_size(vec_str!["babca", "bbazb"]), 3);
        assert_eq!(Solution::min_deletion_size(vec_str!["edcba"]), 4);
        assert_eq!(Solution::min_deletion_size(vec_str!["ghi", "def", "abc"]), 0);
    }
}

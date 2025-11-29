// https://leetcode.cn/problems/ZbAuEH/
// LCP 57. 打地鼠
pub struct Solution;
impl Solution {
    pub fn get_maximum_number(moles: Vec<Vec<i32>>) -> i32 {
        let mut moles = moles;
        moles.sort_by_key(|a| std::cmp::Reverse(a[0]));
        let mut n = moles.len();
        let mut s = 0;
        for i in (0..n).rev() {
            if moles[i][0] == 0 {
                if moles[i][1] == 1 && moles[i][2] == 1 {
                    s += 1;
                }
                moles.pop();
            } else {
                break;
            }
        }
        moles.push(vec![0, 1, 1]);
        n = moles.len();
        let mut cp = vec![0; n];
        let mut gp = vec![0; n + 1];
        for i in (0..n).rev() {
            for j in (i + 1)..n {
                if moles[i][0] - moles[j][0] >= 4 {
                    cp[i] = cp[i].max(gp[j] + 1);
                    break;
                } else {
                    let d = (moles[i][1] - moles[j][1]).abs() + (moles[i][2] - moles[j][2]).abs();
                    if d <= moles[i][0] - moles[j][0] {
                        cp[i] = cp[i].max(cp[j] + 1);
                    }
                }
            }
            gp[i] = gp[i + 1].max(cp[i]);
        }
        gp[0] + s
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn get_maximum_number() {
        assert_eq!(
            Solution::get_maximum_number(vec_vec![[1, 1, 0], [2, 0, 1], [4, 2, 2]]),
            2
        );
        assert_eq!(
            Solution::get_maximum_number(vec_vec![[2, 0, 2], [5, 2, 0], [4, 1, 0], [1, 2, 1], [3, 0, 2]]),
            3
        );
        assert_eq!(Solution::get_maximum_number(vec_vec![[0, 1, 0], [0, 0, 1]]), 0);
    }
}

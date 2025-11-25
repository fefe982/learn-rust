// https://leetcode.cn/problems/3aqs1c/
// LCP 65. 舒适的湿度
pub struct Solution;
impl Solution {
    pub fn un_suitability(operate: Vec<i32>) -> i32 {
        let mx = (operate.iter().max().cloned().unwrap_or(0) * 2) as usize;
        let mut f = vec![i32::MAX; mx + 1];
        f[0] = 0;
        for x in operate {
            let mut nf = vec![i32::MAX; mx + 1];
            let x = x as usize;
            for (d, m) in f.into_iter().enumerate() {
                if m == i32::MAX {
                    continue;
                }
                if d + x <= mx {
                    nf[d + x] = nf[d + x].min(m.max((d + x) as i32))
                }
                if d >= x {
                    nf[d - x] = nf[d - x].min(m);
                } else {
                    nf[0] = nf[0].min(m + (x - d) as i32);
                }
            }
            f = nf;
        }
        f.into_iter().min().unwrap_or(0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_un_suitability() {
        assert_eq!(Solution::un_suitability(vec![5, 3, 7]), 8);
        assert_eq!(Solution::un_suitability(vec![20, 10]), 20);
    }
}

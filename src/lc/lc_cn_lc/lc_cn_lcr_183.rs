// https://leetcode.cn/problems/hua-dong-chuang-kou-de-zui-da-zhi-lcof/
// LCR 183. 望远镜中最高的海拔
pub struct Solution;
impl Solution {
    pub fn max_altitude(heights: Vec<i32>, limit: i32) -> Vec<i32> {
        super::super::lc02::lc_239::Solution::max_sliding_window(heights, limit)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_altitude() {
        assert_eq!(
            Solution::max_altitude(vec![14, 2, 27, -5, 28, 13, 39], 3),
            vec![27, 27, 28, 28, 39]
        );
    }
}

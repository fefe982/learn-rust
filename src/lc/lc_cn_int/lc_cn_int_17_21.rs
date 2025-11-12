// https://leetcode.cn/problems/volume-of-histogram-lcci/
// 面试题 17.21. Volume of Histogram LCCI
pub struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }
        let mut hh: Vec<(i32, usize)> = vec![];
        let mut sum = 0;
        for idx in 0..height.len() {
            let mut l = 0;
            while let Some(&(h, i)) = hh.last() {
                if h > height[idx] {
                    sum += (height[idx] - l) * (idx - i - 1) as i32;
                    break;
                }
                sum += (h - l) * (idx - i - 1) as i32;
                l = h;
                hh.pop();
            }
            hh.push((height[idx], idx));
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trap() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}

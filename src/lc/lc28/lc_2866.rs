// https://leetcode.com/problems/beautiful-towers-ii/
// 2866. Beautiful Towers II
pub struct Solution;
impl Solution {
    pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
        let mut stk: Vec<(i32, i32)> = vec![(0, -1)];
        let mut left = vec![0; max_heights.len()];
        let mut last = 0;
        for i in 0..max_heights.len() {
            let height = max_heights[i];
            let mut lastidx = i as i32;
            let mut lastheight = height;
            while let Some(&(h, j)) = stk.last() {
                last -= (lastheight - height) as i64 * (lastidx - j) as i64;
                lastidx = j as i32;
                lastheight = h;
                if height <= h {
                    stk.pop();
                } else {
                    break;
                }
            }
            stk.push((height, i as i32));
            last += height as i64;
            left[i] = last;
        }
        stk.clear();
        stk.push((0, max_heights.len() as i32));
        last = 0;
        let mut max = 0;
        for i in (0..max_heights.len()).rev() {
            let height = max_heights[i];
            let mut lastidx = i as i32;
            let mut lastheight = height;
            while let Some(&(h, j)) = stk.last() {
                last -= (lastheight - height) as i64 * (j - lastidx) as i64;
                lastidx = j as i32;
                lastheight = h;
                if height <= h {
                    stk.pop();
                } else {
                    break;
                }
            }
            stk.push((height, i as i32));
            max = max.max(last + left[i]);
            last += height as i64;
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_sum_of_heights() {
        assert_eq!(Solution::maximum_sum_of_heights(vec![5, 3, 4, 1, 1]), 13);
        assert_eq!(Solution::maximum_sum_of_heights(vec![6, 5, 3, 9, 2, 7]), 22);
        assert_eq!(Solution::maximum_sum_of_heights(vec![3, 2, 5, 5, 2, 3]), 18);
    }
}

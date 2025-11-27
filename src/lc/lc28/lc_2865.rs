// https://leetcode.com/problems/beautiful-towers-i/
// 2865. Beautiful Towers I
pub struct Solution;
impl Solution {
    fn sum(heights: impl Iterator<Item = i32>) -> Vec<i64> {
        let mut s = vec![];
        let mut stack = vec![];
        let mut sum = 0;
        for (i, h) in heights.enumerate() {
            let mut li = i;
            while let Some(&(si, sh)) = stack.last() {
                if sh >= h {
                    sum -= (li - si) as i64 * (sh - h) as i64;
                    stack.pop();
                    li = si;
                } else {
                    break;
                }
            }
            stack.push((li, h));
            sum += h as i64;
            s.push(sum);
        }
        s
    }
    pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
        let left = Self::sum(max_heights.iter().copied());
        let right = Self::sum(max_heights.iter().copied().rev());
        let mut ans = 0;
        for (i, (l, r)) in left.into_iter().zip(right.into_iter().rev()).enumerate() {
            if l + r - max_heights[i] as i64 > ans {
                ans = l + r - max_heights[i] as i64;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_sum_of_heights() {
        assert_eq!(Solution::maximum_sum_of_heights(vec![5, 3, 4, 1, 1]), 13);
        assert_eq!(Solution::maximum_sum_of_heights(vec![6, 5, 3, 9, 2, 7]), 22);
    }
}

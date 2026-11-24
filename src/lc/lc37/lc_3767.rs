// https://leetcode.com/problems/maximize-points-after-choosing-k-tasks/
// 3767. Maximize Points After Choosing K Tasks
pub struct Solution;
impl Solution {
    pub fn max_points(technique1: Vec<i32>, technique2: Vec<i32>, k: i32) -> i64 {
        let mut technique1 = technique1;
        let mut sum = 0;
        for i in 0..technique1.len() {
            sum += technique2[i] as i64;
            technique1[i] -= technique2[i];
        }
        technique1.sort_by_key(|&x| std::cmp::Reverse(x));
        let k = k as usize;
        for i in 0..k {
            sum += technique1[i] as i64;
        }
        for i in k..technique1.len() {
            if technique1[i] > 0 {
                sum += technique1[i] as i64;
            } else {
                break;
            }
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_points() {
        assert_eq!(Solution::max_points(vec![5, 2, 10], vec![10, 3, 8], 2), 22);
        assert_eq!(Solution::max_points(vec![10, 20, 30], vec![5, 15, 25], 2), 60);
        assert_eq!(Solution::max_points(vec![1, 2, 3], vec![4, 5, 6], 0), 15);
    }
}

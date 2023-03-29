// https://leetcode.com/problems/reducing-dishes/
// 1402. Reducing Dishes
pub struct Solution;
impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort();
        if satisfaction[satisfaction.len() - 1] <= 0 {
            return 0;
        }
        let mut res = 0;
        let mut sum = 0;
        let mut add = 0;
        for &s in satisfaction.iter().rev() {
            sum += add + s;
            add += s;
            if sum > res {
                res = sum;
            } else {
                break;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_satisfaction() {
        assert_eq!(Solution::max_satisfaction(vec![-1, -8, 0, 5, -9]), 14);
        assert_eq!(Solution::max_satisfaction(vec![4, 3, 2]), 20);
        assert_eq!(Solution::max_satisfaction(vec![-1, -4, -5]), 0);
    }
}

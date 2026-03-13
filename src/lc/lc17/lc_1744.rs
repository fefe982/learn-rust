// https://leetcode.com/problems/can-you-eat-your-favorite-candy-on-your-favorite-day/
// 1744. Can You Eat Your Favorite Candy on Your Favorite Day
pub struct Solution;
impl Solution {
    pub fn can_eat(candies_count: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut ans = Vec::with_capacity(queries.len());
        let mut acc = vec![0; candies_count.len() + 1];
        for i in 0..candies_count.len() {
            acc[i + 1] = acc[i] + candies_count[i] as i64;
        }
        for q in queries {
            let (t, d, w) = (q[0], q[1], q[2]);
            let t = t as usize;
            let d = d as i64;
            let w = w as i64;
            let max_day = acc[t + 1] - 1;
            let min_day = acc[t] / w;
            ans.push(min_day <= d && d <= max_day);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn can_eat() {
        assert_eq!(
            Solution::can_eat(vec![7, 4, 5, 3, 8], vec_vec![[0, 2, 2], [4, 2, 4], [2, 13, 1000000000]]),
            [true, false, true]
        );
        assert_eq!(
            Solution::can_eat(
                vec![5, 2, 6, 4, 1],
                vec_vec![[3, 1, 2], [4, 10, 3], [3, 10, 100], [4, 100, 30], [1, 3, 1]]
            ),
            [false, true, true, false, false]
        );
    }
}

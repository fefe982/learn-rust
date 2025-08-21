// https://leetcode.com/problems/two-city-scheduling/description/
// 1029. Two City Scheduling
pub struct Solution;
impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut costs = costs;
        costs.sort_by_key(|x| x[0] - x[1]);
        let mut sum = 0;
        for i in 0..costs.len() / 2 {
            sum += costs[i][0] + costs[costs.len() - i - 1][1];
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn two_city_sched_cost() {
        assert_eq!(
            Solution::two_city_sched_cost(vec_vec![[10, 20], [30, 200], [400, 50], [30, 20]]),
            110
        );
        assert_eq!(
            Solution::two_city_sched_cost(vec_vec![
                [259, 770],
                [448, 54],
                [926, 667],
                [184, 139],
                [840, 118],
                [577, 469]
            ]),
            1859
        );
        assert_eq!(
            Solution::two_city_sched_cost(vec_vec![
                [515, 563],
                [451, 713],
                [537, 709],
                [343, 819],
                [855, 779],
                [457, 60],
                [650, 359],
                [631, 42]
            ]),
            3086
        );
    }
}

// https://leetcode.cn/problems/minimum-cost-homecoming-of-a-robot-in-a-grid/
// 2087. Minimum Cost Homecoming of a Robot in a Grid
pub struct Solution;
impl Solution {
    pub fn min_cost(start_pos: Vec<i32>, home_pos: Vec<i32>, row_costs: Vec<i32>, col_costs: Vec<i32>) -> i32 {
        let (sr, sc) = (start_pos[0], start_pos[1]);
        let (hr, hc) = (home_pos[0], home_pos[1]);

        let mut ans = 0;

        if sr < hr {
            for r in (sr + 1)..=hr {
                ans += row_costs[r as usize];
            }
        } else {
            for r in hr..=(sr - 1) {
                ans += row_costs[r as usize];
            }
        }

        if sc < hc {
            for c in (sc + 1)..=hc {
                ans += col_costs[c as usize];
            }
        } else {
            for c in hc..=(sc - 1) {
                ans += col_costs[c as usize];
            }
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_cost() {
        assert_eq!(
            Solution::min_cost(vec![1, 0], vec![2, 3], vec![5, 4, 3], vec![8, 2, 6, 7]),
            18
        );
        assert_eq!(
            Solution::min_cost(vec![0, 0], vec![0, 0], vec![5], vec![26]),
            0
        );
    }
}
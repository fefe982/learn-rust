// https://leetcode.com/problems/closest-dessert-cost/
// 1774. Closest Dessert Cost
pub struct Solution;
impl Solution {
    pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
        let mut toppings = vec![false; target as usize * 2 + 1];
        toppings[0] = true;
        for &c in &topping_costs {
            for t in (0..=target as usize * 2).rev() {
                if toppings[t as usize] {
                    for j in 1..=2 {
                        let idx = t + j * c as usize;
                        if idx < toppings.len() {
                            toppings[idx] = true;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        let mut ans = 0;
        let mut min_diff = i32::MAX;
        for base in base_costs {
            for i in 0..=target as usize * 2 {
                if toppings[i] {
                    let diff = (base + i as i32 - target).abs();
                    if diff < min_diff || (diff == min_diff && base + (i as i32) < ans) {
                        ans = base + i as i32;
                        min_diff = diff;
                    }
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn closest_cost() {
        assert_eq!(Solution::closest_cost(vec![1, 7], vec![3, 4], 10), 10);
        assert_eq!(Solution::closest_cost(vec![2, 3], vec![4, 5, 100], 18), 17);
        assert_eq!(Solution::closest_cost(vec![3, 10], vec![2, 5], 9), 8);
    }
}

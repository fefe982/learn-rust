// https://leetcode.com/problems/minimum-cost-to-equalize-array/
// 3139. Minimum Cost to Equalize Array
pub struct Solution;
impl Solution {
    pub fn min_cost_to_equalize_array(nums: Vec<i32>, cost1: i32, cost2: i32) -> i32 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut sum = 0;
        let len = nums.len() as i64;
        if len == 1 {
            return 0;
        }
        for n in nums {
            min = min.min(n);
            max = max.max(n);
            sum += n as i64;
        }
        let md = 1_000_000_007i64;
        let cost1 = cost1 as i64;
        let cost2 = cost2 as i64;
        if cost1 * 2 <= cost2 {
            ((len * max as i64 - sum) % md * cost1 % md) as i32
        } else {
            let total = len * max as i64 - sum;
            let mut c2 = (total / 2).min(total - (max - min) as i64);
            let mut c1 = total - c2 * 2;
            if cost1 * (len - 2) > cost2 * (len - 1) {
                c2 += c1 / (len - 2) * (len - 1);
                c1 %= len - 2;
                while (c1 + len) / 2 * cost2 + (c1 + len) % 2 * cost1 < c1 * cost1 {
                    c2 += (c1 + len) / 2;
                    c1 = (c1 + len) % 2;
                }
            }
            println!("{total} {sum} {max} {min} {c2} {c1}");
            ((c2 % md * cost2 + c1 % md * cost1) % md) as i32
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_cost_to_equalize_array() {
        assert_eq!(
            Solution::min_cost_to_equalize_array(vec![1000000, 2, 1, 2, 1000000], 10000, 4000),
            999997965
        );
        assert_eq!(
            Solution::min_cost_to_equalize_array(vec![60, 19, 53, 31, 57], 60, 2),
            90
        );
        assert_eq!(Solution::min_cost_to_equalize_array(vec![1, 14, 14, 15], 2, 1), 20);
        assert_eq!(Solution::min_cost_to_equalize_array(vec![4, 1], 5, 2), 15);
        assert_eq!(Solution::min_cost_to_equalize_array(vec![2, 3, 3, 3, 5], 2, 1), 6);
        assert_eq!(Solution::min_cost_to_equalize_array(vec![3, 5, 3], 1, 3), 4);
    }
}

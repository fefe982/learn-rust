// https://leetcode.com/problems/final-array-state-after-k-multiplication-operations-ii/
// 3266. Final Array State After K Multiplication Operations II
pub struct Solution;
impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        if multiplier == 1 {
            return nums;
        }
        let mut h = std::collections::BinaryHeap::new();
        let mut max = 1;
        for (i, &n) in nums.iter().enumerate() {
            h.push(std::cmp::Reverse((n, i)));
            max = max.max(n);
        }
        let mut k = k;
        let mut nums = nums;
        while k > 0 && max / h.peek().unwrap().0 .0 >= multiplier {
            let std::cmp::Reverse(mut top) = h.pop().unwrap();
            top.0 *= multiplier;
            h.push(std::cmp::Reverse(top));
            k -= 1;
        }
        let mut t = k / nums.len() as i32;
        let mut p1 = 1;
        let mut m = multiplier as i64;
        let md = 1_000_000_007;
        while t > 0 {
            if t & 1 == 1 {
                p1 = (p1 * m) % md;
            }
            m = (m * m) % md;
            t >>= 1;
        }
        let p2 = (p1 * multiplier as i64) % md;
        let mut t = k % nums.len() as i32;
        while let Some(std::cmp::Reverse(top)) = h.pop() {
            if t > 0 {
                nums[top.1] = ((top.0 as i64 * p2) % md) as i32;
                t -= 1;
            } else {
                nums[top.1] = ((top.0 as i64 * p1) % md) as i32;
            }
        }
        nums
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_final_state() {
        assert_eq!(
            Solution::get_final_state(vec![2, 1, 3, 5, 6], 5, 2),
            vec![8, 4, 6, 5, 6]
        );
        assert_eq!(
            Solution::get_final_state(vec![100000, 2000], 2, 1000000),
            [999999307, 999999993]
        );
    }
}

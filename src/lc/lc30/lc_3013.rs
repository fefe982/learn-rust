// https://leetcode.com/problems/divide-an-array-into-subarrays-with-minimum-cost-ii/
// 3013. Divide an Array into Subarrays With Minimum Cost II
pub struct Solution;
impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        let mut hlow = std::collections::BinaryHeap::new();
        let mut hhigh = std::collections::BinaryHeap::new();
        let mut v = vec![false; nums.len()];
        let dist = dist as usize;
        let k = k as usize;
        let mut sum = 0;
        for i in 1..=dist + 1 {
            hlow.push((nums[i], i));
            v[i] = true;
            sum += nums[i] as i64;
            if hlow.len() >= k {
                let (n, idx) = hlow.pop().unwrap();
                v[idx] = false;
                sum -= n as i64;
                hhigh.push((std::cmp::Reverse(n), idx));
            }
        }
        let mut min = sum;
        for i in dist + 2..nums.len() {
            if v[i - dist - 1] {
                hhigh.push((std::cmp::Reverse(nums[i]), i));
                sum -= nums[i - dist - 1] as i64;
                while let Some((std::cmp::Reverse(n), idx)) = hhigh.pop() {
                    if idx >= i - dist {
                        hlow.push((n, idx));
                        v[idx] = true;
                        sum += n as i64;
                        break;
                    }
                }
            } else {
                hlow.push((nums[i], i));
                v[i] = true;
                sum += nums[i] as i64;
                while let Some((n, idx)) = hlow.pop() {
                    if idx >= i - dist {
                        v[idx] = false;
                        sum -= n as i64;
                        hhigh.push((std::cmp::Reverse(n), idx));
                        break;
                    }
                }
            }
            min = min.min(sum)
        }
        min + nums[0] as i64
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_cost() {
        assert_eq!(
            Solution::minimum_cost(
                vec![36, 45, 39, 45, 37, 19, 2, 21, 33, 2, 40, 17, 32, 9, 39, 33, 38, 40, 5, 22],
                12,
                14
            ),
            231
        );
        assert_eq!(Solution::minimum_cost(vec![1, 3, 2, 6, 4, 2], 3, 3), 5);
        assert_eq!(Solution::minimum_cost(vec![10, 1, 2, 2, 2, 1], 4, 3), 15);
        assert_eq!(Solution::minimum_cost(vec![10, 8, 18, 9], 3, 1), 36);
    }
}

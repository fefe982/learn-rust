// https://leetcode.com/problems/divide-an-array-into-subarrays-with-minimum-cost-ii/
// 3103. Divide an Array into Subarrays With Minimum Cost II
pub struct Solution;
impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        let mut heap_all = std::collections::BinaryHeap::new();
        let mut heap_low = std::collections::BinaryHeap::new();
        let k = k as usize;
        let dist = dist as usize;
        let mut sum = 0;
        let mut selected = vec![false; nums.len()];
        let mut nselect = 0;
        for i in 1..dist + 2 {
            heap_low.push((nums[i], i));
            sum += nums[i] as i64;
            selected[i] = true;
            nselect += 1;
            if heap_low.len() >= k {
                let (n, idx) = heap_low.pop().unwrap();
                selected[idx] = false;
                sum -= n as i64;
                nselect -= 1;
                heap_all.push((std::cmp::Reverse(n), idx));
            }
        }
        let mut ans = sum;
        let mut bound = 1;
        while bound + dist + 1 < nums.len() {
            if selected[bound] == true {
                sum -= nums[bound] as i64;
                nselect -= 1;
                while let Some((n, idx)) = heap_all.pop() {
                    if idx > bound {
                        heap_low.push((n.0, idx));
                        sum += n.0 as i64;
                        selected[idx] = true;
                        nselect += 1;
                        break;
                    }
                }
            }
            bound += 1;
            heap_low.push((nums[bound + dist], bound + dist));
            sum += nums[bound + dist] as i64;
            nselect += 1;
            selected[bound + dist] = true;
            while nselect >= k {
                let (n, idx) = heap_low.pop().unwrap();
                selected[idx] = false;
                if idx >= bound {
                    sum -= n as i64;
                    heap_all.push((std::cmp::Reverse(n), idx));
                    nselect -= 1;
                }
            }
            ans = ans.min(sum);
        }
        ans + nums[0] as i64
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_cost() {
        assert_eq!(Solution::minimum_cost(vec![2, 6, 3, 8, 3, 1, 1], 3, 4), 4);
        assert_eq!(Solution::minimum_cost(vec![1, 3, 2, 6, 4, 2], 3, 3), 5);
        assert_eq!(Solution::minimum_cost(vec![10, 1, 2, 2, 2, 1], 4, 3), 15);
        assert_eq!(Solution::minimum_cost(vec![10, 8, 18, 9], 3, 1), 36);
    }
}

// https://leetcode.com/problems/earliest-second-to-mark-indices-ii/
// 3049. Earliest Second to Mark Indices II
pub struct Solution;
impl Solution {
    fn check(
        nums: &Vec<i32>,
        change_indices: &Vec<i32>,
        first: &Vec<usize>,
        mut time_actual: i64,
        time_bound: i64,
    ) -> bool {
        let mut h = std::collections::BinaryHeap::new();
        let mut mark = 0;
        for i in (0..time_bound as usize).rev() {
            if time_actual <= time_bound {
                return true;
            }
            if first[change_indices[i] as usize] == usize::MAX || first[change_indices[i] as usize] < i {
                mark += 1;
                continue;
            }
            let diff = nums[change_indices[i] as usize - 1] - 1;
            time_actual -= diff as i64;
            h.push(std::cmp::Reverse(diff));
            if h.len() > mark {
                time_actual += h.pop().unwrap().0 as i64;
                mark += 1;
            }
        }
        time_actual <= time_bound
    }
    pub fn earliest_second_to_mark_indices(nums: Vec<i32>, change_indices: Vec<i32>) -> i32 {
        let mut first = vec![usize::MAX; nums.len() + 1];
        for (i, &c) in change_indices.iter().enumerate() {
            if nums[c as usize - 1] > 0 && first[c as usize] == usize::MAX {
                first[c as usize] = i;
            }
        }
        let mut low = 1;
        let mut high = change_indices.len() as i64;
        let max_time = nums.iter().map(|&x| x as i64).sum::<i64>() + nums.len() as i64;
        if !Self::check(&nums, &change_indices, &first, max_time, high) {
            return -1;
        }
        if max_time <= low {
            return low as i32;
        }
        while low + 1 < high {
            let mid = (low + high) / 2;
            if Self::check(&nums, &change_indices, &first, max_time, mid) {
                high = mid;
            } else {
                low = mid;
            }
        }
        high as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn earliest_second_to_mark_indices() {
        assert_eq!(
            Solution::earliest_second_to_mark_indices(
                vec![5, 1, 3, 2, 2, 5],
                vec![3, 2, 2, 3, 1, 1, 3, 4, 2, 3, 4, 2, 5, 6, 5, 3, 6, 5, 3]
            ),
            15
        );
        assert_eq!(
            Solution::earliest_second_to_mark_indices(
                vec![5, 3, 2, 0, 3, 5],
                vec![4, 3, 6, 5, 6, 5, 3, 6, 4, 1, 2, 3, 6, 1]
            ),
            13
        );
        assert_eq!(
            Solution::earliest_second_to_mark_indices(vec![0, 0, 3, 0], vec![2, 2, 2, 2, 2, 2, 3, 1]),
            7
        );
        assert_eq!(
            Solution::earliest_second_to_mark_indices(vec![3, 2, 3], vec![1, 3, 2, 2, 2, 2, 3]),
            6
        );
        assert_eq!(
            Solution::earliest_second_to_mark_indices(vec![0, 0, 1, 2], vec![1, 2, 1, 2, 1, 2, 1, 2]),
            7
        );
        assert_eq!(
            Solution::earliest_second_to_mark_indices(vec![1, 2, 3], vec![1, 2, 3]),
            -1
        );
    }
}

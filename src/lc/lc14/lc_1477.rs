// https://leetcode.com/problems/find-two-non-overlapping-sub-arrays-each-with-target-sum/
// 1477. Find Two Non-overlapping Sub-arrays Each With Target Sum
pub struct Solution;
impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let mut q = std::collections::VecDeque::new();
        let mut mins = i32::MAX;
        let mut min = i32::MAX;
        let mut i = 0;
        let mut j = 0;
        let mut sum = 0;
        while j < arr.len() {
            sum += arr[j];
            while sum > target {
                sum -= arr[i];
                i += 1;
            }
            if sum == target {
                let l = (j - i + 1) as i32;
                q.push_back((j, l));
                while let Some(&(p, pl)) = q.front() {
                    if p < i {
                        mins = mins.min(pl);
                        q.pop_front();
                    } else {
                        break;
                    }
                }
                if mins < i32::MAX {
                    min = min.min(mins + l);
                }
            }
            j += 1;
        }
        if min == i32::MAX {
            -1
        } else {
            min
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_sum_of_lengths() {
        assert_eq!(Solution::min_sum_of_lengths(vec![3, 2, 2, 4, 3], 3), 2);
        assert_eq!(Solution::min_sum_of_lengths(vec![7, 3, 4, 7], 7), 2);
        assert_eq!(Solution::min_sum_of_lengths(vec![4, 3, 2, 6, 2, 3, 4], 6), -1);
    }
}

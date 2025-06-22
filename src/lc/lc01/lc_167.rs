// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
// 167. Two Sum II - Input Array Is Sorted
pub struct Solution;
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut j = 1;
        while j + 1 < numbers.len() && numbers[i] + numbers[j] < target {
            j += 1;
        }
        if numbers[i] + numbers[j] == target {
            return vec![i as i32 + 1, j as i32 + 1];
        }
        while numbers[i] + numbers[j] < target {
            i += 1;
        }
        if numbers[i] + numbers[j] == target {
            return vec![i as i32 + 1, j as i32 + 1];
        }
        loop {
            while numbers[i] + numbers[j] > target {
                j -= 1;
            }
            if numbers[i] + numbers[j] == target {
                return vec![i as i32 + 1, j as i32 + 1];
            }
            while numbers[i] + numbers[j] < target {
                i += 1;
            }
            if numbers[i] + numbers[j] == target {
                return vec![i as i32 + 1, j as i32 + 1];
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn two_sum() {
        assert_eq!(Solution::two_sum(vec![5, 25, 75], 100), vec![2, 3]);
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
    }
}

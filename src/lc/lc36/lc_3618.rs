// https://leetcode.com/problems/split-array-by-prime-indices/
// 3618. Split Array by Prime Indices
pub struct Solution;
impl Solution {
    pub fn split_array(nums: Vec<i32>) -> i64 {
        if nums.len() < 3 {
            return nums.into_iter().map(|i| i as i64).sum::<i64>().abs();
        }
        let mut prime = vec![1; nums.len()];
        prime[0] = -1;
        prime[1] = -1;
        for i in 2.. {
            if i * i > nums.len() {
                break;
            }
            if prime[i] != 1 {
                continue;
            }
            for j in (i * i..nums.len()).step_by(i) {
                prime[j] = -1;
            }
        }
        nums.into_iter()
            .zip(prime.into_iter())
            .map(|(a, b)| a as i64 * b as i64)
            .sum::<i64>()
            .abs()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn split_array() {
        assert_eq!(Solution::split_array(vec![2, 3, 4]), 1);
        assert_eq!(Solution::split_array(vec![-1, 5, 7, 0]), 3);
    }
}

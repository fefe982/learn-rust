// https://leetcode.com/problems/sum-of-imbalance-numbers-of-all-subarrays/
// 2763. Sum of Imbalance Numbers of All Subarrays
pub struct Solution;
impl Solution {
    pub fn sum_imbalance_numbers(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            let mut set = std::collections::BTreeSet::new();
            set.insert(nums[i]);
            let mut imb = 0;
            for j in i + 1..nums.len() {
                let nj = nums[j];
                if !set.contains(&nj) {
                    let prev = set.range(..nj).last();
                    let next = set.range(nj..).next();
                    match (prev, next) {
                        (Some(p), Some(n)) => {
                            if nj - p > 1 {
                                imb += 1;
                            }
                            if n - nj > 1 {
                                imb += 1;
                            }
                            if n - p > 1 {
                                imb -= 1;
                            }
                        }
                        (Some(p), None) => {
                            if nj - p > 1 {
                                imb += 1;
                            }
                        }
                        (None, Some(n)) => {
                            if n - nj > 1 {
                                imb += 1;
                            }
                        }
                        _ => (),
                    }
                }
                set.insert(nj);
                ans += imb;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_imbalance_numbers() {
        assert_eq!(Solution::sum_imbalance_numbers(vec![2, 3, 1, 4]), 3);
        assert_eq!(Solution::sum_imbalance_numbers(vec![1, 3, 3, 3, 5]), 8);
    }
}

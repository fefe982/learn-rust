// https://leetcode.com/problems/greatest-sum-divisible-by-three/
// 1262. Greatest Sum Divisible by Three
pub struct Solution;
impl Solution {
    fn push(q: &mut Vec<i32>, n: i32) {
        if q.len() == 2 && q[1] > n {
            q.pop();
        }
        if q.len() == 0 || q.len() == 1 && q[0] <= n {
            q.push(n);
        } else if q.len() == 1 {
            q.insert(0, n);
        }
    }
    fn sum(q: &Vec<i32>, n: i32) -> i32 {
        if q.len() < n as usize {
            i32::MAX
        } else if n == 1 {
            q[0]
        } else {
            q[0] + q[1]
        }
    }
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut one = Vec::new();
        let mut two = Vec::new();
        let mut sum = 0;
        nums.into_iter().for_each(|n| {
            sum += n;
            if n % 3 == 1 {
                Self::push(&mut one, n);
            } else if n % 3 == 2 {
                Self::push(&mut two, n);
            }
        });
        sum - if sum % 3 == 0 {
            0
        } else if sum % 3 == 1 {
            Self::sum(&one, 1).min(Self::sum(&two, 2))
        } else {
            Self::sum(&one, 2).min(Self::sum(&two, 1))
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_sum_div_three() {
        assert_eq!(Solution::max_sum_div_three(vec![3, 6, 5, 1, 8]), 18);
        assert_eq!(Solution::max_sum_div_three(vec![4]), 0);
        assert_eq!(Solution::max_sum_div_three(vec![1, 2, 3, 4, 4]), 12);
    }
}

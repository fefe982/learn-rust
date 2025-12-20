// https://leetcode.cn/problems/qie-fen-shu-zu/
// LCP 14. 切分数组
pub struct Solution;
impl Solution {
    pub fn split_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let max = (*nums.iter().max().unwrap()) as usize;
        let mut prime = Vec::with_capacity(max);
        let mut minprime = vec![usize::MAX; max + 1];
        for i in 2..=max {
            if minprime[i] == usize::MAX {
                minprime[i] = prime.len();
                prime.push(i);
            }
            for j in 0..prime.len() {
                if prime[j] * i > max {
                    break;
                }
                minprime[prime[j] * i] = j;
                if i % prime[j] == 0 {
                    break;
                }
            }
        }
        let mut dp = vec![n; prime.len()];
        let mut ans = 0;
        for i in 0..n {
            let mut x = nums[i] as usize;
            while x > 1 {
                let j = minprime[x];
                dp[j] = dp[j].min(ans);
                x /= prime[j];
            }
            ans = i + 1;
            x = nums[i] as usize;
            while x > 1 {
                let j = minprime[x];
                ans = ans.min(dp[j] + 1);
                x /= prime[j];
            }
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn split_array() {
        assert_eq!(Solution::split_array(vec![326614, 489921]), 1);
        assert_eq!(Solution::split_array(vec![2, 3, 3, 2, 3, 3]), 2);
        assert_eq!(Solution::split_array(vec![2, 3, 5, 7]), 4);
    }
}

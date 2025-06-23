// https://leetcode.com/problems/maximum-candies-allocated-to-k-children
// 2226. Maximum Candies Allocated to K Children
pub struct Solution;
impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let mut sum: i64 = 0;
        let mut max: i32 = 0;
        let mut mc = 0;
        for &i in &candies {
            sum += i as i64;
            match i.cmp(&max) {
                std::cmp::Ordering::Greater => {
                    max = i;
                    mc = 1;
                }
                std::cmp::Ordering::Equal => mc += 1,
                std::cmp::Ordering::Less => {}
            }
        }
        if sum < k {
            return 0;
        }
        if mc as i64 >= k {
            return max;
        }
        let mut min = 1;
        while min + 1 < max {
            let mid = (min + max) / 2;
            if candies.iter().map(|i| (i / mid) as i64).sum::<i64>() >= k {
                min = mid;
            } else {
                max = mid;
            }
        }
        min
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_candies() {
        assert_eq!(Solution::maximum_candies(vec![5, 8, 6], 3), 5);
        assert_eq!(Solution::maximum_candies(vec![2, 5], 11), 0);
    }
}

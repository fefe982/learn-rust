// https://leetcode.com/problems/distribute-candies-to-people/
// 1103. Distribute Candies to People
pub struct Solution;
impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let mut ans = vec![0; num_people as usize];
        let n = ((((1 + 8 * candies as i64) as f64).sqrt() - 1.0) / 2.0) as i32;
        println!("{n}");
        let ci = candies - n * (n + 1) / 2;
        let k = n / num_people;
        let ki = n % num_people;
        for j in 0..num_people {
            if j < ki {
                ans[j as usize] = (k + 1) * (j * 2 + 2 + k * num_people) / 2;
            } else if j == ki {
                ans[j as usize] = k * (j * 2 + 2 + (k - 1) * num_people) / 2 + ci;
            } else {
                ans[j as usize] = k * (j * 2 + 2 + (k - 1) * num_people) / 2;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_distribute_candies() {
        assert_eq!(Solution::distribute_candies(7, 4), [1, 2, 3, 1]);
        assert_eq!(Solution::distribute_candies(10, 3), [5, 2, 3]);
    }
}

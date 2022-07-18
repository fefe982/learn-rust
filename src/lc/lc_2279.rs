// https://leetcode.com/problems/maximum-bags-with-full-capacity-of-rocks/
// 2279. Maximum Bags With Full Capacity of Rocks
pub struct Solution;
impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
        let mut space: Vec<i32> = capacity.iter().zip(rocks).map(|(x, y)| x - y).collect();
        space.sort_unstable();
        let mut count = 0;
        let mut sum = 0;
        for s in space {
            sum += s;
            if sum > additional_rocks {
                break;
            }
            count += 1
        }
        count
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_bags() {
        assert_eq!(
            Solution::maximum_bags(vec![2, 3, 4, 5], vec![1, 2, 4, 4], 2),
            3
        );
        assert_eq!(
            Solution::maximum_bags(vec![10, 2, 2], vec![2, 2, 0], 100),
            3
        )
    }
}

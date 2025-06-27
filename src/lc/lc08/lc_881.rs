// https://leetcode.com/problems/boats-to-save-people/
// 881. Boats to Save People
pub struct Solution;
impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort_unstable();
        let mut low = 0;
        let mut high = people.len() - 1;
        let mut cnt = 0;
        while low < high {
            if people[low] + people[high] <= limit {
                low += 1;
                high -= 1;
                cnt += 1;
            } else {
                cnt += 1;
                high -= 1;
            }
        }
        if low == high {
            cnt += 1;
        }
        cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_rescue_boats() {
        assert_eq!(Solution::num_rescue_boats(vec![1, 2], 3), 1);
        assert_eq!(Solution::num_rescue_boats(vec![3, 2, 2, 1], 3), 3);
        assert_eq!(Solution::num_rescue_boats(vec![3, 5, 3, 4], 5), 4);
    }
}

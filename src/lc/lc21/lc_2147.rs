// https://leetcode.com/problems/number-of-ways-to-divide-a-long-corridor/
// 2147. Number of Ways to Divide a Long Corridor
pub struct Solution;
impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut last = 0;
        let mut count: i64 = 1;
        let mut cs = 0;
        for (i, c) in corridor.chars().enumerate() {
            if c == 'S' {
                cs += 1;
                if cs % 2 == 1 {
                    if last != 0 {
                        count = count * (i - last) as i64 % MOD;
                    }
                } else {
                    last = i
                }
            }
        }
        if cs > 0 && cs % 2 == 0 {
            count as i32
        } else {
            0
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_of_ways() {
        assert_eq!(Solution::number_of_ways("P".to_string()), 0);
        assert_eq!(Solution::number_of_ways("SSPPSPS".to_string()), 3);
        assert_eq!(Solution::number_of_ways("PPSPSP".to_string()), 1);
        assert_eq!(Solution::number_of_ways("S".to_string()), 0);
    }
}

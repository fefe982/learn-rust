// https://leetcode.com/problems/number-of-people-aware-of-a-secret/
// 2327. Number of People Aware of a Secret
pub struct Solution;
impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        let modulo = 1000000007;
        let mut vec = Vec::with_capacity(n as usize);
        vec.push(1);
        let mut partial_sum = 0;
        let mut people_know = 1;
        for i in 1..n {
            if i - delay >= 0 {
                partial_sum = (partial_sum + vec[(i - delay) as usize]) % modulo;
                if i - forget >= 0 {
                    partial_sum = (partial_sum - vec[(i - forget) as usize] + modulo) % modulo;
                    people_know = (people_know - vec[(i - forget) as usize] + modulo) % modulo;
                }
            }
            vec.push(partial_sum);
            people_know = (people_know + partial_sum) % modulo;
        }
        people_know
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn people_aware_of_secret() {
        assert_eq!(Solution::people_aware_of_secret(6, 2, 4), 5);
        assert_eq!(Solution::people_aware_of_secret(4, 1, 3), 6);
    }
}

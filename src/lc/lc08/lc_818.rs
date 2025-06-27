// https://leetcode.com/problems/race-car/
// 818. Race Car
pub struct Solution;
impl Solution {
    fn helper(target: i32, mem: &mut std::collections::HashMap<i32, i32>) -> i32 {
        if target == 0 {
            return 0;
        }
        if let Some(v) = mem.get(&target) {
            return *v;
        }
        let mut c = i32::MAX;
        let mut i = 1;
        let mut m = 1;
        while i < target {
            let mut j = 0;
            let mut n = 0;
            while j < i {
                c = c.min(Self::helper(target - i + j, mem) + m + n + 2);
                n += 1;
                j = j * 2 + 1;
            }
            m += 1;
            i = i * 2 + 1;
        }
        if i == target {
            c = c.min(m);
        } else {
            c = c.min(m + Self::helper(i - target, mem) + 1);
        }
        mem.insert(target, c);
        c
    }
    pub fn racecar(target: i32) -> i32 {
        let mut mem = std::collections::HashMap::new();
        let a = Self::helper(target, &mut mem);
        a
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_racecar() {
        assert_eq!(Solution::racecar(5), 7);
        assert_eq!(Solution::racecar(2), 4);
        assert_eq!(Solution::racecar(3), 2);
        assert_eq!(Solution::racecar(6), 5);
    }
}

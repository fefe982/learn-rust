// https://leetcode.com/problems/restore-the-array/
// 1416. Restore The Array
pub struct Solution;
#[derive(Copy, Clone)]
struct IMode {
    i: i64,
}
impl IMode {
    fn new(i: i64) -> Self {
        Self { i: i as i64 }
    }
    fn val(self) -> i32 {
        self.i as i32
    }
}
impl std::ops::Add<IMode> for IMode {
    type Output = IMode;
    fn add(self, rhs: IMode) -> IMode {
        return IMode::new((self.i + rhs.i) % 1000000007);
    }
}
impl Solution {
    fn num_arr(s: &[u8], k: i64, cache: &mut std::collections::HashMap<usize, IMode>) -> IMode {
        if s.len() == 0 {
            return IMode::new(1);
        }
        if s[0] == b'0' {
            return IMode::new(0);
        }
        if let Some(cnt) = cache.get(&s.len()) {
            return *cnt;
        }
        let mut cnt = IMode::new(0);
        let mut val = 0;
        for i in 0..s.len() {
            val = val * 10 + ((s[i] - b'0') as i64);
            if val <= k {
                cnt = cnt + Self::num_arr(&s[i + 1..], k, cache);
            } else {
                break;
            }
        }
        cache.insert(s.len(), cnt);
        cnt
    }
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        Self::num_arr(
            s.as_bytes(),
            k as i64,
            &mut std::collections::HashMap::new(),
        )
        .val()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_of_arrays() {
        assert_eq!(Solution::number_of_arrays(String::from("1000"), 10000), 1);
        assert_eq!(Solution::number_of_arrays(String::from("1000"), 10), 0);
        assert_eq!(Solution::number_of_arrays(String::from("1317"), 2000), 8);
    }
}

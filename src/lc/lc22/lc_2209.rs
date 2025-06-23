// https://leetcode.com/problems/minimum-white-tiles-after-covering-with-carpets/
// 2209. Minimum White Tiles After Covering With Carpets
pub struct Solution;
impl Solution {
    pub fn minimum_white_tiles(floor: String, num_carpets: i32, carpet_len: i32) -> i32 {
        let mut v = vec![];
        let mut cnt = 0;
        let floor = floor.as_bytes();
        for &c in floor {
            if c == b'1' {
                cnt += 1;
            }
            v.push(cnt);
        }
        let carpet_len = carpet_len as usize;
        for _ in 0..num_carpets {
            let mut nv = vec![0; v.len()];
            for j in carpet_len..v.len() {
                nv[j] = v[j - carpet_len].min(nv[j - 1] + (floor[j] - b'0') as i32);
            }
            v = nv;
        }
        *v.last().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_white_tiles() {
        assert_eq!(Solution::minimum_white_tiles("10110101".to_string(), 2, 2), 2);
        assert_eq!(Solution::minimum_white_tiles("1111".to_string(), 2, 3), 0);
    }
}

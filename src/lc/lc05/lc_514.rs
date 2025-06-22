// https://leetcode.com/problems/freedom-trail/
// 514. Freedom Trail
pub struct Solution;
impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let mut m = std::collections::HashMap::<u8, Vec<usize>>::new();
        for (i, &c) in ring.as_bytes().iter().enumerate() {
            m.entry(c).or_default().push(i);
        }
        let l = ring.len();
        let mut pos = vec![(0, 0)];
        for k in key.as_bytes() {
            let mut npos = vec![];
            for &np in m.get(k).unwrap() {
                let mut low = usize::MAX;
                for &(p, c) in pos.iter() {
                    let m = (np + l - p) % l;
                    let m = m.min(l - m);
                    low = low.min(c + m + 1);
                }
                npos.push((np, low));
            }
            pos = npos;
        }
        pos.iter().fold(usize::MAX, |min, &(_, c)| min.min(c)) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_rotate_steps() {
        assert_eq!(
            Solution::find_rotate_steps("godding".to_string(), "gd".to_string()),
            4
        );
        assert_eq!(
            Solution::find_rotate_steps("godding".to_string(), "godding".to_string()),
            13
        );
    }
}

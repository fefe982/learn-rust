// https://leetcode.com/problems/stamping-the-sequence/
// 936. Stamping The Sequence
pub struct Solution;
impl Solution {
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        let stamp = stamp.chars().collect::<Vec<char>>();
        let mut target = target.chars().collect::<Vec<char>>();
        let mut res = vec![];
        loop {
            let mut changed = false;
            let mut done = true;
            for i in 0..(target.len() - stamp.len() + 1) {
                let mut matched = true;
                let mut wdone = true;
                for j in 0..stamp.len() {
                    if target[i + j] == '?' {
                        continue;
                    }
                    wdone = false;
                    done = false;
                    if target[i + j] != stamp[j] {
                        matched = false;
                        break;
                    }
                }
                if wdone {
                    continue;
                }
                if matched {
                    changed = true;
                    for j in 0..stamp.len() {
                        target[i + j] = '?';
                    }
                    res.push(i as i32);
                }
            }
            if done {
                res.reverse();
                return res;
            }
            if !changed {
                return vec![];
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn stamp_paper(stamp: &str, target: &str, op: Vec<i32>) -> bool {
        let stamp = stamp.chars().collect::<Vec<char>>();
        let target = target.chars().collect::<Vec<char>>();
        let mut paper = vec!['?'; target.len()];
        for pos in op {
            if pos < 0 || pos as usize + stamp.len() > paper.len() {
                return false;
            }
            for i in 0..stamp.len() {
                paper[pos as usize + i] = stamp[i]
            }
        }
        paper == target
    }
    #[test]
    fn test_moves_to_stamp() {
        for (stamp, target) in [("abc", "ababc"), ("abca", "aabcaca")] {
            assert!(stamp_paper(
                stamp,
                target,
                Solution::moves_to_stamp(stamp.to_string(), target.to_string())
            ));
        }
    }
}

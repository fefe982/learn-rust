// https://leetcode.com/problems/dota2-senate/
// 649. Dota2 Senate
pub struct Solution;
impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut que: Vec<u8> = senate.as_bytes().iter().cloned().collect();
        let l = que.len();
        let mut r_idx = l;
        let mut d_idx = l;
        let mut cur = 0;
        loop {
            let head = que[cur];
            if head == b'R' {
                if d_idx == l {
                    for idx in 0..l {
                        if que[(cur + idx) % l] == b'D' {
                            d_idx = (cur + idx) % l;
                            break;
                        }
                    }
                }
                if d_idx == l {
                    return String::from("Radiant");
                } else {
                    que[d_idx] = b'.';
                    for idx in 0..l {
                        let midx = (d_idx + idx) % l;
                        if midx == cur {
                            return String::from("Radiant");
                        }
                        if que[midx] == b'D' {
                            d_idx = midx;
                            break;
                        }
                    }
                }
            }
            if head == b'D' {
                if r_idx == l {
                    for idx in 0..l {
                        if que[(cur + idx) % l] == b'R' {
                            r_idx = (cur + idx) % l;
                            break;
                        }
                    }
                }
                if r_idx == l {
                    return String::from("Dire");
                } else {
                    que[r_idx] = b'.';
                    for idx in 0..l {
                        let midx = (r_idx + idx) % l;
                        if midx == cur {
                            return String::from("Dire");
                        }
                        if que[midx] == b'R' {
                            r_idx = midx;
                            break;
                        }
                    }
                }
            }
            cur = (cur + 1) % l;
            if cur == r_idx {
                r_idx = l;
            }
            if cur == d_idx {
                d_idx = l;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn predict_party_victory() {
        assert_eq!(
            Solution::predict_party_victory(String::from("DRRDRDRDRDDRDRDRD")),
            String::from("Dire")
        );
        assert_eq!(
            Solution::predict_party_victory(String::from("DDRRR")),
            String::from("Dire")
        );
        assert_eq!(
            Solution::predict_party_victory(String::from("RD")),
            String::from("Radiant")
        );
        assert_eq!(
            Solution::predict_party_victory(String::from("RDD")),
            String::from("Dire")
        );
    }
}

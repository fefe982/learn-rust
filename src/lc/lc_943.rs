// https://leetcode.com/problems/find-the-shortest-superstring/
// 943. Find the Shortest Superstring
pub struct Solution;
impl Solution {
    pub fn shortest_superstring(words: Vec<String>) -> String {
        let mut d = vec![vec![0; words.len()]; words.len()];
        for i in 0..words.len() {
            let wi = words[i].as_bytes();
            for j in 0..words.len() {
                let wj = words[j].as_bytes();
                if i != j {
                    let s = wi.len() - wi.len().min(wj.len());
                    for k in s..wi.len() {
                        if wi[k..] == wj[..wi.len() - k] {
                            d[i][j] = wj.len() + k - wi.len();
                            break;
                        }
                    }
                    if d[i][j] == 0 {
                        d[i][j] = wj.len();
                    }
                }
            }
        }
        let mut dist = vec![vec![0; words.len()]; 1 << words.len()];
        let mut path = vec![vec![usize::MAX; words.len()]; 1 << words.len()];
        for i in 1usize..(1 << words.len()) {
            if i.count_ones() == 1 {
                let idx = i.trailing_zeros() as usize;
                dist[i][idx] = words[idx].len();
            } else {
                let mut mask = 0;
                loop {
                    let mi = i & !mask;
                    if mi == 0 {
                        break;
                    }
                    let j = (mi - 1) & i;
                    mask |= i ^ j;
                    let k = (i ^ j).trailing_zeros() as usize;
                    let mut min = usize::MAX;
                    let mut lmin = 0;
                    for l in 0..words.len() {
                        if dist[j][l] == 0 {
                            continue;
                        }
                        let d = dist[j][l] + d[l][k];
                        if d < min {
                            min = d;
                            lmin = l;
                        }
                    }
                    dist[i][k] = min;
                    path[i][k] = lmin;
                }
            }
        }
        let mut lmin = dist
            .last()
            .unwrap()
            .iter()
            .enumerate()
            .fold(
                (usize::MAX, 0),
                |(min, last), (i, &d)| if d < min { (d, i) } else { (min, last) },
            )
            .1;

        let mut state = (1usize << words.len()) - 1;
        let mut vpath = vec![lmin];
        while state > 0 {
            vpath.push(lmin);
            let s = 1 << lmin;
            lmin = path[state][lmin];
            state -= s;
        }
        vpath
            .into_iter()
            .rev()
            .fold(("".to_string(), 0), |(mut acc, last), idx| {
                if acc.is_empty() {
                    (words[idx].clone(), idx)
                } else {
                    acc.extend(words[idx].chars().skip(words[idx].len() - d[last][idx]));
                    (acc, idx)
                }
            })
            .0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    fn check_shortest_superstring(words: Vec<String>, expected_len: usize, result: &str) -> bool {
        if result.len() != expected_len {
            return false;
        }
        for word in words.iter() {
            if !result.contains(word) {
                return false;
            }
        }
        return true;
    }
    #[test]
    fn test_shortest_superstring() {
        for case in [
            (vec_str!["wmiy", "yarn", "rnnwc", "arnnw", "wcj"], "wmiyarnnwcj"), // (vec_str!["alex", "loves", "leetcode"], "alexlovesleetcode"),
                                                                                // (
                                                                                //     vec_str!["catg", "ctaagt", "gcta", "ttca", "atgcatc"],
                                                                                //     "gctaagttcatgcatc",
                                                                                // ),
        ] {
            assert!(check_shortest_superstring(
                case.0.clone(),
                case.1.len(),
                &Solution::shortest_superstring(case.0.clone()),
            ));
        }
    }
}

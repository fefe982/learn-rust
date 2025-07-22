// https://leetcode.com/problems/minimize-malware-spread-ii/
// 928. Minimize Malware Spread II
pub struct Solution;
impl Solution {
    pub fn min_malware_spread(graph: Vec<Vec<i32>>, initial: Vec<i32>) -> i32 {
        let mut g = std::collections::HashMap::<usize, std::collections::HashSet<usize>>::new();
        let e = std::collections::HashSet::<usize>::new();
        for i in 0..graph.len() {
            for j in 0..graph[i].len() {
                if graph[i][j] == 1 {
                    g.entry(i).or_default().insert(j);
                    g.entry(j).or_default().insert(i);
                }
            }
        }
        let mut initial = initial;
        initial.sort_unstable();
        let uninfected = -1;
        let infect_src = -2;
        let mut infect = vec![uninfected; graph.len()];
        for &i in &initial {
            infect[i as usize] = infect_src;
        }
        let mut r = 0;
        let mut rc = -1;
        for i in initial {
            let mut cnt = vec![0; graph.len()];
            let mut infect = infect.clone();
            let mut q = vec![(i as usize, i as usize)];
            while let Some((qi, si)) = q.pop() {
                if infect[qi] >= 0 {
                    continue;
                }
                infect[qi] = si as i32;
                if si != i as usize {
                    if cnt[si] == -1 {
                        continue;
                    } else {
                        cnt[si] += 1;
                    }
                }
                for &j in g.get(&qi).unwrap_or(&e) {
                    let src = if si == i as usize { j } else { si };
                    if infect[j] == i || infect[j] == si as i32 {
                        continue;
                    }
                    if infect[j] != uninfected {
                        cnt[src] = -1;
                        continue;
                    }
                    q.push((j, src));
                }
            }
            let c = cnt.into_iter().filter(|x| *x > 0).sum::<i32>();
            if c > rc {
                rc = c;
                r = i;
            }
        }
        r
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_malware_spread() {
        assert_eq!(
            Solution::min_malware_spread(vec_vec![[1, 1, 0], [1, 1, 0], [0, 0, 1]], vec![0, 1]),
            0
        );
        assert_eq!(
            Solution::min_malware_spread(vec_vec![[1, 1, 0], [1, 1, 1], [0, 1, 1]], vec![0, 1]),
            1
        );
        assert_eq!(
            Solution::min_malware_spread(
                vec_vec![[1, 1, 0, 0], [1, 1, 1, 0], [0, 1, 1, 1], [0, 0, 1, 1]],
                vec![0, 1]
            ),
            1
        );
    }
}

// https://leetcode.com/problems/minimum-genetic-mutation/
// 433. Minimum Genetic Mutation
pub struct Solution;
impl Solution {
    fn check_gen(e: &str, gene: &str) -> bool {
        let mut diff = 0;
        for i in 0..gene.len() {
            if e.as_bytes()[i] != gene.as_bytes()[i] {
                diff += 1;
            }
            if diff > 1 {
                return false;
            }
        }
        diff == 1
    }
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        let mut q = vec![(start_gene.as_str(), 0)];
        let mut visited = vec![false; bank.len()];
        while let Some((gene, step)) = q.pop() {
            if gene == end_gene {
                return step;
            }
            for i in 0..bank.len() {
                if !visited[i] && Self::check_gen(gene, &bank[i]) {
                    visited[i] = true;
                    q.push((bank[i].as_str(), step + 1));
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_mutation() {
        assert_eq!(
            Solution::min_mutation("AACCGGTT".to_string(), "AACCGGTA".to_string(), vec_str!["AACCGGTA"]),
            1
        );
        assert_eq!(
            Solution::min_mutation(
                "AACCGGTT".to_string(),
                "AAACGGTA".to_string(),
                vec_str!["AACCGGTA", "AACCGCTA", "AAACGGTA"]
            ),
            2
        );
    }
}

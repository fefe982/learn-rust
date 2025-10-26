// https://leetcode.com/problems/sum-of-perfect-square-ancestors/
// 3715. Sum of Perfect Square Ancestors
pub struct Solution;
impl Solution {
    fn get_prime(n: i32) -> Vec<i32> {
        let mut is_prime = vec![true; (n + 1) as usize];
        is_prime[0] = false;
        is_prime[1] = false;
        let mut i = 2;
        while i * i <= n {
            if is_prime[i as usize] {
                let mut j = i * i;
                while j <= n {
                    is_prime[j as usize] = false;
                    j += i;
                }
            }
            i += 1;
        }
        let mut primes = Vec::new();
        for i in 0..is_prime.len() {
            if is_prime[i] {
                primes.push(i as i32);
            }
        }
        primes
    }
    fn remove_square_factors(mut n: i32, primes: &Vec<i32>) -> i32 {
        for &p in primes {
            let p2 = p * p;
            while n % p2 == 0 {
                n /= p2;
            }
            if p2 > n {
                break;
            }
        }
        n
    }
    fn count_square(
        graph: &Vec<Vec<i32>>,
        nums: &Vec<i32>,
        node: usize,
        parent: usize,
        counter: &mut std::collections::HashMap<i32, i32>,
    ) -> i64 {
        let val = nums[node];
        let mut cnt = 0;
        if let Some(&count) = counter.get(&val) {
            cnt += count as i64;
        }
        *counter.entry(val).or_insert(0) += 1;
        for &neighbor in &graph[node] {
            if neighbor as usize != parent {
                cnt += Self::count_square(graph, nums, neighbor as usize, node, counter);
            }
        }
        *counter.get_mut(&val).unwrap() -= 1;
        cnt
    }
    pub fn sum_of_ancestors(n: i32, edges: Vec<Vec<i32>>, nums: Vec<i32>) -> i64 {
        let primes = Self::get_prime(316);
        let nums = nums
            .into_iter()
            .map(|x| Self::remove_square_factors(x, &primes))
            .collect::<Vec<i32>>();
        let mut graph = vec![vec![]; n as usize];
        for edge in edges {
            graph[edge[0] as usize].push(edge[1]);
            graph[edge[1] as usize].push(edge[0]);
        }
        Self::count_square(&graph, &nums, 0, usize::MAX, &mut std::collections::HashMap::new())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn sum_of_ancestors() {
        assert_eq!(
            Solution::sum_of_ancestors(3, vec_vec![[0, 1], [1, 2]], vec![2, 8, 2]),
            3
        );
        assert_eq!(
            Solution::sum_of_ancestors(3, vec_vec![[0, 1], [0, 2]], vec![1, 2, 4]),
            1
        );
        assert_eq!(
            Solution::sum_of_ancestors(4, vec_vec![[0, 1], [0, 2], [1, 3]], vec![1, 2, 9, 4]),
            2
        );
    }
}

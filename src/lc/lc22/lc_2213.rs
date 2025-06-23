// https://leetcode.com/problems/longest-substring-of-one-repeating-character/
// 2213. Longest Substring of One Repeating Character
pub struct Solution;
impl Solution {
    fn push(
        seg_map: &mut std::collections::BTreeMap<usize, (usize, char)>,
        seg_heap: &mut std::collections::BinaryHeap<(usize, usize)>,
        s: usize,
        e: usize,
        c: char,
    ) {
        seg_map.insert(s, (e, c));
        seg_heap.push((e - s, s));
    }
    pub fn longest_repeating(s: String, query_characters: String, query_indices: Vec<i32>) -> Vec<i32> {
        let mut seg_map = std::collections::BTreeMap::new();
        let mut last_char = ',';
        let mut last_pos = 0;
        let mut seg_heap = std::collections::BinaryHeap::new();
        for (i, c) in s.chars().chain(['.']).enumerate() {
            if c != last_char {
                if last_char != ',' {
                    Self::push(&mut seg_map, &mut seg_heap, last_pos, i, last_char);
                }
                last_char = c;
                last_pos = i;
            }
        }
        let mut ans = Vec::with_capacity(query_indices.len());
        for (qi, qc) in query_indices.into_iter().zip(query_characters.chars()) {
            let qi = qi as usize;
            let (&ss, &(se, sc)) = seg_map.range(0..=qi).next_back().unwrap();
            if sc != qc {
                if ss < qi && qi + 1 < se {
                    seg_map.remove(&ss);
                    Self::push(&mut seg_map, &mut seg_heap, ss, qi, sc);
                    Self::push(&mut seg_map, &mut seg_heap, qi, qi + 1, qc);
                    Self::push(&mut seg_map, &mut seg_heap, qi + 1, se, sc);
                } else {
                    let mut ns = qi;
                    let mut ne = qi + 1;
                    let mut lefts = ss;
                    let mut lefte = se;
                    if ss == qi {
                        if let Some((&prevs, &(_, prevc))) = seg_map.range(0..qi).next_back() {
                            if prevc == qc {
                                seg_map.remove(&prevs);
                                ns = prevs;
                            }
                        }
                        lefts += 1;
                    }
                    if qi + 1 == se {
                        if let Some(&(nexte, nextc)) = seg_map.get(&se) {
                            if nextc == qc {
                                seg_map.remove(&se);
                                ne = nexte;
                            }
                        }
                        lefte -= 1;
                    }
                    seg_map.remove(&ss);
                    Self::push(&mut seg_map, &mut seg_heap, ns, ne, qc);
                    if lefte > lefts {
                        Self::push(&mut seg_map, &mut seg_heap, lefts, lefte, sc);
                    }
                }
            }
            while let Some(&(len, pos)) = seg_heap.peek() {
                if let Some(&(e, _)) = seg_map.get(&pos) {
                    if e - pos == len {
                        ans.push(len as i32);
                        break;
                    }
                }
                seg_heap.pop();
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_repeating() {
        assert_eq!(
            Solution::longest_repeating("babacc".to_string(), "bcb".to_string(), vec![1, 3, 3]),
            [3, 3, 4]
        );
        assert_eq!(
            Solution::longest_repeating("abyzz".to_string(), "aa".to_string(), vec![2, 1]),
            [2, 3]
        );
    }
}

// https://leetcode.com/problems/reconstruct-itinerary/
// 332. Reconstruct Itinerary
pub struct Solution;
impl Solution {
    fn search_itinerary<'a>(
        tmap: &mut std::collections::HashMap<&str, std::collections::BTreeMap<&str, i32>>,
        tv: &std::collections::HashMap<&str, Vec<&'a str>>,
        res: &mut Vec<&'a str>,
        l: usize,
    ) -> bool {
        if res.len() == 0 {
            return false;
        }
        let from = res[res.len() - 1];
        if let Some(tov) = tv.get(from) {
            for &to in tov {
                if *tmap.get(from).unwrap().get(to).unwrap() == 0 {
                    continue;
                }
                res.push(to);
                *tmap.get_mut(from).unwrap().get_mut(to).unwrap() -= 1;
                if res.len() == l + 1 || Self::search_itinerary(tmap, tv, res, l) {
                    return true;
                }
                *tmap.get_mut(from).unwrap().get_mut(to).unwrap() += 1;
                res.pop();
            }
        }
        false
    }
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut tmap =
            std::collections::HashMap::<&str, std::collections::BTreeMap<&str, i32>>::new();
        let l = tickets.len();
        for t in tickets.iter() {
            *tmap.entry(&t[0]).or_default().entry(&t[1]).or_default() += 1;
        }
        let tv: std::collections::HashMap<&str, Vec<&str>> = tmap
            .iter()
            .map(|(&k, v)| (k, v.iter().map(|(&s, _)| s).collect()))
            .collect();
        let mut res = vec!["JFK"];
        Self::search_itinerary(&mut tmap, &tv, &mut res, l);
        res.into_iter().map(|x| x.to_owned()).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn top_k_frequent() {
        assert_eq!(
            Solution::find_itinerary(vec_vec_str![["JFK", "KUL"], ["JFK", "NRT"], ["NRT", "JFK"]]),
            vec_str!["JFK", "NRT", "JFK", "KUL"]
        );
        assert_eq!(
            Solution::find_itinerary(vec_vec_str![
                ["MUC", "LHR"],
                ["JFK", "MUC"],
                ["SFO", "SJC"],
                ["LHR", "SFO"]
            ]),
            vec_str!["JFK", "MUC", "LHR", "SFO", "SJC"]
        );
        assert_eq!(
            Solution::find_itinerary(vec_vec_str![
                ["JFK", "SFO"],
                ["JFK", "ATL"],
                ["SFO", "ATL"],
                ["ATL", "JFK"],
                ["ATL", "SFO"]
            ]),
            vec_str!["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"]
        );
    }
}

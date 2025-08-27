// https://leetcode.com/problems/smallest-sufficient-team/
// 1125. Smallest Sufficient Team
pub struct Solution;
use std::collections::HashMap;
impl Solution {
    fn get_skill_set(skill_map: &HashMap<String, i32>, skills: &Vec<String>) -> i32 {
        let mut res = 0;
        for skill in skills {
            res += 1 << skill_map.get(skill).unwrap()
        }
        res
    }
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        let skill_map: HashMap<String, i32> = req_skills.into_iter().zip(0..).collect();
        let people_vec: Vec<i32> = people
            .iter()
            .map(|x| Self::get_skill_set(&skill_map, x))
            .collect();
        let sz = 1usize << skill_map.len();
        let mut min_people = vec![i32::MAX; sz];
        min_people[0] = 0;
        let mut from_group = vec![0; sz];
        let mut add_people = vec![0; sz];
        for (idx, &people_skill) in people_vec.iter().enumerate() {
            for n in 0..sz {
                if min_people[n] == i32::MAX {
                    continue;
                }
                let next_set = n | people_skill as usize;
                if min_people[n] + 1 < min_people[next_set] {
                    min_people[next_set] = min_people[n] + 1;
                    from_group[next_set] = n;
                    add_people[next_set] = idx as i32;
                }
            }
        }
        let mut res = vec![];
        let mut path = sz - 1;
        while path != 0 {
            res.push(add_people[path]);
            path = from_group[path];
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::super::helpers;
    use super::*;
    #[test]
    fn smallest_sufficient_team() {
        assert_eq!(
            Solution::smallest_sufficient_team(
                helpers::make_string_vec(["java", "nodejs", "reactjs"]),
                vec![
                    helpers::make_string_vec(["java"]),
                    helpers::make_string_vec(["nodejs"]),
                    helpers::make_string_vec(["nodejs", "reactjs"])
                ]
            ),
            vec![2, 0]
        );
        assert_eq!(
            Solution::smallest_sufficient_team(
                helpers::make_string_vec([
                    "algorithms",
                    "math",
                    "java",
                    "reactjs",
                    "csharp",
                    "aws"
                ]),
                vec![
                    helpers::make_string_vec(["algorithms", "math", "java"]),
                    helpers::make_string_vec(["algorithms", "math", "reactjs"]),
                    helpers::make_string_vec(["java", "csharp", "aws"]),
                    helpers::make_string_vec(["reactjs", "csharp"]),
                    helpers::make_string_vec(["csharp", "math"]),
                    helpers::make_string_vec(["aws", "java"])
                ]
            ),
            vec![2, 1]
        );
    }
}

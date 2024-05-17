// https://leetcode.com/problems/delete-duplicate-folders-in-system/
// 1948. Delete Duplicate Folders in System
pub struct Solution;
impl Solution {
    fn find_tree(
        v: &Vec<std::collections::BTreeMap<String, usize>>,
        parent: String,
        node: usize,
        mark: &mut Vec<bool>,
        subtree: &mut std::collections::HashMap<String, usize>,
    ) -> String {
        let mut res = parent;
        let mut sub = "".to_string();
        for (folder, &idx) in v[node].iter() {
            sub.push(';');
            sub.push_str(&Self::find_tree(v, folder.clone(), idx, mark, subtree));
        }
        if !sub.is_empty() {
            if let Some(&id) = subtree.get(&sub) {
                mark[id] = true;
                mark[node] = true;
            } else {
                subtree.insert(sub.clone(), node);
            }
        }
        res.push('(');
        res.push_str(&sub);
        res.push(')');
        res
    }
    fn collect_tree(
        v: &Vec<std::collections::BTreeMap<String, usize>>,
        parent: &mut Vec<String>,
        node: usize,
        mark: &mut Vec<bool>,
        result: &mut Vec<Vec<String>>,
    ) {
        if !parent.is_empty() {
            result.push(parent.clone());
        }
        for (folder, &idx) in v[node].iter() {
            if !mark[idx] {
                parent.push(folder.clone());
                Self::collect_tree(v, parent, idx, mark, result);
                parent.pop();
            }
        }
    }
    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut v = vec![std::collections::BTreeMap::<String, usize>::new()];
        for path in paths {
            let mut parent = 0;
            for folder in path {
                if let Some(child) = v[parent].get(&folder) {
                    parent = *child;
                } else {
                    let np = v.len();
                    v[parent].insert(folder.clone(), np);
                    v.push(std::collections::BTreeMap::<String, usize>::new());
                    parent = np;
                }
            }
        }
        let mut mark = vec![false; v.len()];
        let mut subtree = std::collections::HashMap::<String, usize>::new();
        Self::find_tree(&v, "".to_string(), 0, &mut mark, &mut subtree);
        let mut result = vec![];
        Self::collect_tree(&v, &mut vec![], 0, &mut mark, &mut result);
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_delete_duplicate_folder() {
        assert_eq!(
            Solution::delete_duplicate_folder(vec_vec_str![["a"], ["c"], ["d"], ["a", "b"], ["c", "b"], ["d", "a"]]),
            vec_vec_str![["d"], ["d", "a"]]
        );
        assert_eq!(
            Solution::delete_duplicate_folder(vec_vec_str![
                ["a"],
                ["c"],
                ["a", "b"],
                ["c", "b"],
                ["a", "b", "x"],
                ["a", "b", "x", "y"],
                ["w"],
                ["w", "y"]
            ]),
            vec_vec_str![["a"], ["a", "b"], ["c"], ["c", "b"]]
        );
    }
}

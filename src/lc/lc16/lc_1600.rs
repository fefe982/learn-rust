// https://leetcode.com/problems/throne-inheritance/
// 1600. Throne Inheritance
struct TreeNode {
    name: String,
    dead: bool,
    children: Vec<usize>,
}

impl TreeNode {
    fn new(name: String) -> Self {
        Self {
            name,
            dead: false,
            children: Vec::new(),
        }
    }
}
pub struct ThroneInheritance {
    name_map: std::collections::HashMap<String, usize>,
    nodes: Vec<TreeNode>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ThroneInheritance {
    pub fn new(king_name: String) -> Self {
        Self {
            name_map: std::collections::HashMap::from([(king_name.clone(), 0)]),
            nodes: vec![TreeNode::new(king_name)],
        }
    }

    pub fn birth(&mut self, parent_name: String, child_name: String) {
        let n = self.nodes.len();
        self.name_map.insert(child_name.clone(), n);
        self.nodes.push(TreeNode::new(child_name));
        self.nodes[self.name_map[&parent_name]].children.push(n);
    }

    pub fn death(&mut self, name: String) {
        self.nodes[self.name_map[&name]].dead = true;
    }

    fn get_inher(&self, root: usize, pre: Vec<String>) -> Vec<String> {
        let mut ret = pre;
        if !self.nodes[root].dead {
            ret.push(self.nodes[root].name.clone());
        }
        for &child in &self.nodes[root].children {
            ret = self.get_inher(child, ret);
        }
        ret
    }
    pub fn get_inheritance_order(&self) -> Vec<String> {
        self.get_inher(0, vec![])
    }
}

/**
 * Your ThroneInheritance object will be instantiated and called as such:
 * let obj = ThroneInheritance::new(kingName);
 * obj.birth(parentName, childName);
 * obj.death(name);
 * let ret_3: Vec<String> = obj.get_inheritance_order();
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_throne_order() {
        let func = vec_str![
            "ThroneInheritance",
            "birth",
            "birth",
            "birth",
            "birth",
            "birth",
            "birth",
            "getInheritanceOrder",
            "death",
            "getInheritanceOrder"
        ];
        let args = vec_vec_str![
            ["king"],
            ["king", "andy"],
            ["king", "bob"],
            ["king", "catherine"],
            ["andy", "matthew"],
            ["bob", "alex"],
            ["bob", "asha"],
            ["null"],
            ["bob"],
            ["null"]
        ];
        let expect = vec_vec_str![
            ["null"],
            ["null"],
            ["null"],
            ["null"],
            ["null"],
            ["null"],
            ["null"],
            ["king", "andy", "matthew", "bob", "alex", "asha", "catherine"],
            ["null"],
            ["king", "andy", "matthew", "alex", "asha", "catherine"]
        ];
        let mut obj = ThroneInheritance::new("".to_string());
        for ((fun, arg), exp) in func.into_iter().zip(args.into_iter()).zip(expect.into_iter()) {
            if fun == "ThroneInheritance".to_string() {
                obj = ThroneInheritance::new(arg[0].to_string());
            } else if fun == "birth".to_string() {
                obj.birth(arg[0].to_string(), arg[1].to_string());
            } else if fun == "death".to_string() {
                obj.death(arg[0].to_string());
            } else if fun == "getInheritanceOrder".to_string() {
                assert_eq!(obj.get_inheritance_order(), exp);
            } else {
                unreachable!()
            }
        }
    }
}

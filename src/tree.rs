pub struct Node {
    elem: String,
    children: Vec<Box<Node>>,
    path_from_root: String,
}

impl Node {
    pub fn new(elem: String) -> Self {
        Node {
            elem: elem,
            children: Vec::new(),
            path_from_root: "".to_string(),
        }
    }

    pub fn get_elem(&self) -> String {
        self.elem.clone()
    }

    pub fn get_children(&self) -> &Vec<Box<Node>> {
        &(self.children)
    }

    pub fn get_path(&self) -> String {
        self.path_from_root.clone()
    }

    pub fn insert_child(&mut self, mut child: Node) {
        if self.path_from_root.len() > 0 {
            child.path_from_root = format!("{},{}", self.path_from_root, self.elem);
        } else {
            child.path_from_root = format!("{}", self.elem);
        }
        self.children.push(Box::new(child))
    }

    pub fn new_child(&mut self, child_str: String) {
        let child = Node::new(child_str);
        self.insert_child(child);
    }

    pub fn new_child_of(&mut self, parent: String, child: String) -> bool {
        if self.elem == parent {
            self.new_child(child);
            true
        } else if self.children.len() > 0 {
            let mut ret_val = false;
            for node in self.children.iter_mut() {
                ret_val = ret_val || node.new_child_of(parent.clone(), child.clone());
            }
            return ret_val;
        } else {
            false
        }
    }

    pub fn find_node(&self, value: &str) -> Option<&Node> {
        if self.elem == value.to_string() {
            Some(&self)
        } else if self.children.len() > 0 {
            let matches: Vec<&Node> = self
                .children
                .iter()
                .filter_map(|n| n.find_node(value))
                .collect();
            if matches.len() > 0 {
                Some(matches[0])
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.children.len() > 0 {
            write!(
                f,
                "{} ({}): {:?}",
                self.elem, self.path_from_root, self.children
            )
        } else {
            write!(f, "{} ({})", self.elem, self.path_from_root)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Node;

    #[test]
    fn new() {
        let node = Node::new("A".to_string());
        assert_eq!(node.elem, "A".to_string());
        assert_eq!(node.elem, "A".to_string());
    }

    #[test]
    fn get_elem() {
        let node = Node::new("A".to_string());
        assert_eq!(node.get_elem(), "A".to_string());
        assert_eq!(node.get_elem(), "A".to_string());
    }

    #[test]
    fn new_child() {
        let mut node = Node::new("AA".to_string());
        node.new_child("A".to_string());
        node.new_child("B".to_string());
        assert_eq!(node.children[0].elem, "A".to_string());
        assert_eq!(node.children[1].elem, "B".to_string());
    }

    #[test]
    fn get_children() {
        let mut node = Node::new("AA".to_string());
        node.new_child("A".to_string());
        node.new_child("B".to_string());
        let children = node.get_children();
        assert_eq!(children[0].elem, node.children[0].elem);
        assert_eq!(children[1].elem, "B".to_string());
        assert_eq!(node.children[0].elem, "A".to_string());
    }

    #[test]
    fn insert_child() {
        let mut node = Node::new("AA".to_string());
        node.insert_child(Node::new("A".to_string()));
        node.insert_child(Node::new("B".to_string()));
        assert_eq!(node.children[0].elem, "A".to_string());
        assert_eq!(node.children[1].elem, "B".to_string());
    }

    #[test]
    fn new_child_of() {
        let mut node = Node::new("O".to_string());
        node.new_child("A".to_string());
        node.new_child("B".to_string());
        assert_eq!(node.new_child_of("A".to_string(), "AA".to_string()), true);
        assert_eq!(
            node.new_child_of("bad".to_string(), "BBB".to_string()),
            false
        );
        assert_eq!(node.children[0].children[0].elem, "AA".to_string());
    }

    #[test]
    fn find_node() {
        let mut node = Node::new("O".to_string());
        node.new_child("A".to_string());
        node.new_child("B".to_string());
        node.new_child_of("B".to_string(), "BB".to_string());
        assert_eq!(
            node.find_node("B").unwrap().children[0].elem,
            "BB".to_string()
        );
    }

    #[test]
    fn get_path() {
        let mut node = Node::new("COM".to_string());
        node.new_child("A".to_string());
        node.new_child("B".to_string());
        node.new_child_of("A".to_string(), "AA".to_string());
        let node_AA = node.find_node("AA").unwrap();
        assert_eq!(node_AA.get_path(), "COM,A".to_string())
    }
}

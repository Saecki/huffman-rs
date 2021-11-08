use std::cmp::max;
use std::collections::HashMap;

pub struct Tree<'a> {
    input: &'a str,
    char_count: HashMap<char, usize>,
    char_code: HashMap<char, String>,
    node: Option<Node>,
}

impl<'a> Tree<'a> {
    pub fn from(input: &'a str) -> Self {
        // count chars
        let mut char_count = HashMap::<char, usize>::new();
        for c in input.chars() {
            match char_count.get_mut(&c) {
                Some(v) => {
                    *v += 1;
                }
                None => {
                    char_count.insert(c, 1);
                }
            }
        }

        // create nodes
        let mut nodes: Vec<_> = char_count
            .iter()
            .map(|(k, v)| {
                Node::Leaf(LeafNode {
                    char: *k,
                    count: *v,
                })
            })
            .collect();

        // construct tree
        while nodes.len() > 1 {
            nodes.sort_by_key(|n| n.count());

            let a = nodes.remove(0);
            let b = nodes.remove(0);
            let n = BranchNode {
                count: a.count() + b.count(),
                a: Box::new(a),
                b: Box::new(b),
            };
            nodes.push(Node::Branch(n));
        }

        // encode chars
        let node = nodes.into_iter().next();
        let mut char_code = HashMap::new();
        if let Some(n) = &node {
            match n {
                Node::Branch(_) => n.encode_chars(String::new(), &mut char_code),
                Node::Leaf(l) => {
                    char_code.insert(l.char, String::from("1"));
                }
            }
        }

        Self {
            input,
            char_count,
            char_code,
            node,
        }
    }

    pub fn width(&self) -> usize {
        self.node.as_ref().map_or(0, |n| n.width())
    }

    pub fn height(&self) -> usize {
        self.node.as_ref().map_or(0, |n| n.height())
    }

    pub fn char_count(&self) -> &HashMap<char, usize> {
        &self.char_count
    }

    pub fn char_code(&self) -> &HashMap<char, String> {
        &self.char_code
    }

    pub fn node(&self) -> Option<&Node> {
        self.node.as_ref()
    }

    pub fn encode(&self, sep: &str) -> String {
        let mut output = String::new();
        let mut iter = self.input.chars().peekable();
        while let Some(c) = iter.next() {
            if let Some(s) = self.char_code.get(&c) {
                output.push_str(s);
                if iter.peek().is_some() {
                    output.push_str(sep)
                }
            }
        }
        return output;
    }

    pub fn encoded_len(&self) -> usize {
        let mut len = 0;
        for c in self.input.chars() {
            if let Some(s) = self.char_code.get(&c) {
                len += s.len();
            }
        }
        len
    }
}

#[derive(Debug)]
pub enum Node {
    Branch(BranchNode),
    Leaf(LeafNode),
}

impl Node {
    pub fn count(&self) -> usize {
        match self {
            Self::Branch(b) => b.count,
            Self::Leaf(l) => l.count,
        }
    }

    pub fn width(&self) -> usize {
        match self {
            Self::Branch(n) => 1 + n.a.width() + n.b.width(),
            Self::Leaf(_) => 1,
        }
    }

    pub fn left_width(&self) -> usize {
        match self {
            Self::Branch(n) => n.a.width(),
            Self::Leaf(_) => 0,
        }
    }

    pub fn right_width(&self) -> usize {
        match self {
            Self::Branch(n) => n.b.width(),
            Self::Leaf(_) => 0,
        }
    }

    pub fn height(&self) -> usize {
        match self {
            Self::Branch(n) => max(n.a.height(), n.b.height()) + 1,
            Self::Leaf(_) => 1,
        }
    }

    fn encode_chars(&self, mut path: String, table: &mut HashMap<char, String>) {
        match self {
            Self::Branch(n) => {
                let path_a = format!("{}{}", path, '0');
                n.a.encode_chars(path_a, table);
                path.push('1');
                n.b.encode_chars(path, table);
            }
            Self::Leaf(n) => {
                table.insert(n.char, path);
            }
        }
    }
}

#[derive(Debug)]
pub struct BranchNode {
    pub count: usize,
    pub a: Box<Node>,
    pub b: Box<Node>,
}

#[derive(Debug)]
pub struct LeafNode {
    pub char: char,
    pub count: usize,
}

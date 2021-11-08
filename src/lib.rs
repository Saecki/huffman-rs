use std::cmp::max;
use std::ops::{Deref, DerefMut};

pub struct Tree<'a> {
    input: &'a str,
    chars: Chars,
    node: Option<Node>,
}

impl<'a> Tree<'a> {
    pub fn from(input: &'a str) -> Self {
        // count chars
        let mut chars = Chars::new();
        for c in input.chars() {
            match chars.get_mut(c) {
                Some(v) => {
                    v.count += 1;
                }
                None => {
                    chars.push(CharEntry {
                        char: c,
                        count: 1,
                        code: String::new(),
                    });
                }
            }
        }

        // create nodes
        let mut nodes: Vec<_> = chars
            .iter()
            .map(|e| {
                Node::Leaf(LeafNode {
                    char: e.char,
                    count: e.count,
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
        if let Some(n) = &node {
            match n {
                Node::Branch(_) => n.encode_chars(String::new(), &mut chars),
                Node::Leaf(l) => {
                    if let Some(e) = chars.get_mut(l.char) {
                        e.code = String::from("1");
                    }
                }
            }
        }

        Self { input, chars, node }
    }

    pub fn width(&self) -> usize {
        self.node.as_ref().map_or(0, |n| n.width())
    }

    pub fn height(&self) -> usize {
        self.node.as_ref().map_or(0, |n| n.height())
    }

    pub fn chars(&self) -> &Chars {
        &self.chars
    }

    pub fn node(&self) -> Option<&Node> {
        self.node.as_ref()
    }

    pub fn encode(&self, sep: &str) -> String {
        let mut output = String::new();
        let mut iter = self.input.chars().peekable();
        while let Some(c) = iter.next() {
            if let Some(e) = self.chars.get(c) {
                output.push_str(&e.code);
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
            if let Some(e) = self.chars.get(c) {
                len += e.code.len();
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

    fn encode_chars(&self, mut path: String, chars: &mut Chars) {
        match self {
            Self::Branch(n) => {
                let path_a = format!("{}{}", path, '0');
                n.a.encode_chars(path_a, chars);
                path.push('1');
                n.b.encode_chars(path, chars);
            }
            Self::Leaf(n) => {
                if let Some(e) = chars.get_mut(n.char) {
                    e.code = path;
                }
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

pub struct Chars(Vec<CharEntry>);

#[derive(Debug)]
pub struct CharEntry {
    pub char: char,
    pub count: usize,
    pub code: String,
}

impl Deref for Chars {
    type Target = Vec<CharEntry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Chars {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Chars {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn get(&self, char: char) -> Option<&CharEntry> {
        self.iter().find(|e| e.char == char)
    }

    pub fn get_mut(&mut self, char: char) -> Option<&mut CharEntry> {
        self.iter_mut().find(|e| e.char == char)
    }
}

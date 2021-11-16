use std::cmp::{max, min};
use std::process::exit;

use huffman_lib::{Node, Tree};

fn main() {
    let mut args = std::env::args().skip(1);
    let input = match args.next() {
        Some(s) => s,
        None => {
            eprintln!("No input string");
            exit(1);
        }
    };

    let tree = Tree::from(input);

    println!();
    println!("Characters:");
    for e in tree.chars().iter() {
        println!("{}{: >6} {}", e.char, e.count, e.code);
    }

    let mut canvas = Canvas::new();
    if let Some(n) = tree.node() {
        paint_tree(&mut canvas, n, 0, n.left_width());
    }
    println!();
    println!("Tree:");
    for l in &canvas.lines {
        println!("{}", l);
    }

    println!();
    println!("Encoded:");
    println!("{}", tree.encode(" "));

    println!();
    println!("Length:");
    println!("{}", tree.encoded_len());
}

struct Canvas {
    lines: Vec<String>,
}

impl Canvas {
    fn new() -> Self {
        Self { lines: Vec::new() }
    }

    fn char(&mut self, x: usize, y: usize, char: char) {
        let line = match self.lines.get_mut(y) {
            Some(l) => l,
            None => {
                for _ in self.lines.len()..=y {
                    self.lines.push(String::new());
                }
                &mut self.lines[y]
            }
        };

        if line.len() <= x {
            for _ in line.len()..x {
                line.push(' ');
            }
            line.push(char);
        } else {
            let mut byte_index = None;
            for (i, (b, _)) in line.char_indices().enumerate() {
                if i == y {
                    byte_index = Some(b);
                    break;
                }
            }
            if let Some(b) = byte_index {
                line.remove(b);
                line.insert(b, char);
            }
        }
    }

    fn string(&mut self, x: usize, y: usize, string: &str) {
        let line = match self.lines.get_mut(y) {
            Some(l) => l,
            None => {
                for _ in self.lines.len()..=y {
                    self.lines.push(String::new());
                }
                &mut self.lines[y]
            }
        };

        if line.len() < x {
            for _ in line.len()..x {
                line.push(' ');
            }
            line.push_str(string);
        } else {
            for (i, c) in string.chars().enumerate() {
                self.char(x + i, y, c);
            }
        }
    }

    fn left_line(&mut self, x: usize, y1: usize, y2: usize) {
        let ys = min(y1, y2);
        let ye = max(y1, y2);

        let mut xo = ye - ys + 1;
        for y in ys..=ye {
            xo -= 1;
            self.char(x + xo, y, '/');
        }
    }

    fn right_line(&mut self, x: usize, y1: usize, y2: usize) {
        let ys = min(y1, y2);
        let ye = max(y1, y2);

        let mut xo = 0;
        for i in ys..=ye {
            self.char(x + xo, i, '\\');
            xo += 1;
        }
    }
}

fn paint_tree(canvas: &mut Canvas, node: &Node, x: usize, y: usize) {
    match node {
        Node::Branch(n) => {
            let y1 = y - 1 - n.a.right_width();
            let y2 = y - 1;
            let ax = x + (y2 - y1) + 2;
            canvas.left_line(x + 1, y1, y2);
            paint_tree(canvas, &n.a, ax, y1);

            canvas.string(x, y, &n.count.to_string());

            let y1 = y + 1;
            let y2 = y + 1 + n.b.left_width() * 1;
            let ax = x + (y2 - y1) + 2;
            canvas.right_line(x + 1, y1, y2);
            paint_tree(canvas, &n.b, ax, y2);
        }
        Node::Leaf(n) => {
            canvas.string(x, y, &format!("{} -- {}", n.count.to_string(), n.char));
        }
    }
}

#![feature(box_syntax, box_patterns)]
// https://doc.rust-lang.org/book/box-syntax-and-patterns.html

pub enum Tree {
    Empty,
    Leaf(i32),
    Node(Box<Tree>, Box<Tree>)
}
/*
pub fn example() {
    let one   = Tree::Leaf(1);
    let two   = Tree::Leaf(2);
    let three = Tree::Leaf(3);
    let n     = Tree::Node(box two, box three);
    let t     = Tree::Node(box one, box n);
}
*/

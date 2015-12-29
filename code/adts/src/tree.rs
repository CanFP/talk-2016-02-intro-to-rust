#![feature(box_syntax, box_patterns)]

enum Tree {
    Empty,
    Leaf(i32),
    Node(Box<Tree>, Box<Tree>)
}

fn main() {
    let one   = Tree::Leaf(1);
    let two   = Tree::Leaf(2);
    let three = Tree::Leaf(3);
    let n     = Tree::Node(Box::new(two), Box::new(three));
    let t     = Tree::Node(Box::new(one), Box::new(n));

    let x = dfs(t, 2);
}

fn dfs(r: Tree, x: i32) -> Tree { // Convert to option return type
    match r {
        Tree::Leaf(i)            => r,
        Tree::Node(box a, box b) => a,
        _                        => Tree::Leaf(0),
    }
}

#![feature(box_syntax, box_patterns)]

enum Tree {
    Empty,
    Leaf(i32),
    Node(Tree, Tree)
}

fn main() {
    let one   = Tree::Leaf(1);
    let two   = Tree::Leaf(2);
    let three = Tree::Leaf(3);
    let n     = Tree::Node(two, three);
    let t     = Tree::Node(one, n);

    let x = dfs(t, 2);
}

fn dfs(r: Tree, x: i32) -> Tree { // Convert to option return type
    match r {
        Tree::Leaf(i)    => r,
        Tree::Node(a, b) => a,
        _                => Tree::Leaf(0),
    }
}

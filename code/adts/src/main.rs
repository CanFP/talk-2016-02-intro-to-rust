#![feature(box_syntax, box_patterns)]
//https://doc.rust-lang.org/book/box-syntax-and-patterns.html

enum Tree {
    Empty,
    Leaf(i32),
    Node(Box<Tree>, Box<Tree>)
}

fn main() {
    let one   = Tree::Leaf(1);
    let two   = Tree::Leaf(2);
    let three = Tree::Leaf(3);
    //let n     = Tree::Node(Box::new(two), Box::new(three));
    let n     = Tree::Node(box two, box three);
    let t     = Tree::Node(box one, box n);

    //dfs(t, 2);
}
/*
fn dfs(Tree: t, i32: n) -> Tree {
    match movement {
        Direction::N(n) => new_pos();
        Command::Download(web, path) => println!("Going to download {}", web),
        Command::Run(path)           => println!("Going to run {}", path),
        _                            => println!("Going to do .. something else")
    }
}
*/

#![feature(box_syntax, box_patterns)]

mod tree;

enum Bool {
    True,
    False
}

enum Length {
    Metres(i32),
    Feet(i32)
}

enum Either<A, B> {
    Left(A),
    Right(B)
}

fn main() {
    let t = Bool::True;
    let f = Bool::False;
    let l: Either<i32, f32> = Either::Right(6.28);
    let two   = tree::Tree::Leaf(2);
    let three = tree::Tree::Leaf(3);
    let n     = tree::Tree::Node(Box::new(two), Box::new(three));
}

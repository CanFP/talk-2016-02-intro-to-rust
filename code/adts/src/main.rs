#![feature(box_syntax, box_patterns)]

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
}

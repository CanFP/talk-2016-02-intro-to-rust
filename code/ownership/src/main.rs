fn main() {
    let mut v1 = vec![1,2,3,4];
    v1 = foo(v1);
    println!("{:?}", v1);
}

fn foo(v: Vec<i32>) -> Vec<i32> {
    return v
}

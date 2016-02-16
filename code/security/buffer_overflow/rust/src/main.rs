use std::env;
 
// Second attempt

// src/main.rs:10:20: 10:27 error: cannot move out of indexed content [E0507]
// src/main.rs:10     strcpy(buffer, args[1]);
//                                   ^~~~~~~
// src/main.rs:18:9: 18:12 error: cannot borrow immutable argument `dst` as mutable
// src/main.rs:18         dst[i] = c;
//                        ^~~
// src/main.rs:16:11: 16:14 help: to make the argument mutable, use `mut` as shown:
// src/main.rs:   fn strcpy(mut dst: [u8; 10], src: String) {

fn main() {
    let args: Vec<_> = env::args().collect();
    let hello = "hello";
    let mut buffer = ['\0' as u8; 10];

    strcpy(&buffer, &args[1]);

    println!("hello:      {}", hello);
    println!("buffer:     {:?}", buffer);
}

fn strcpy(dst: &mut [u8; 10], src: &String) {
    println!("dst before: {:?}", dst);
    for (i, c) in src.bytes().enumerate() {
        dst[i] = c;
    };
    println!("dst after:  {:?}", dst);
}

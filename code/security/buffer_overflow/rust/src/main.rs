use std::env;
 
// First pass

//fn main() {
    //let args: Vec<_> = env::args().collect();
    //let hello = "hello";
    //let buffer: [u8; 10] = ['\0' as u8; 10];

    //strcpy(buffer, args[1]);

    //println!("hello: {}", hello);
    //println!("buffer: {:?}", buffer);
//}

//fn strcpy(dst: [u8; 10], src: String) {
    //for (i, c) in src.bytes().enumerate() {
        //dst[i] = c;
    //};
//}


// Second pass
// Make dst immutable

//fn main() {
    //let args: Vec<_> = env::args().collect();
    //let hello = "hello";
    //let buffer: [u8; 10] = ['\0' as u8; 10];

    //strcpy(buffer, &args[1]);

    //println!("hello: {}", hello);
    //println!("buffer: {:?}", buffer);
//}

//fn strcpy(mut dst: [u8; 10], src: &String) {
    //for (i, c) in src.bytes().enumerate() {
        //dst[i] = c;
    //};
//}


// Third pass
// Make strcpy take a reference

fn main() {
    let args: Vec<_> = env::args().collect();
    let hello = "hello";
    let buffer: [u8; 10] = ['\0' as u8; 10];

    strcpy(buffer, &args[1]);

    println!("hello: {}", hello);
    println!("buffer: {:?}", buffer);
}

fn strcpy(&mut dst: [u8], src: &String) {
    for (i, c) in src.bytes().enumerate() {
        println!("dst {}", dst[i]);
        //dst[i] = c;
    };
}

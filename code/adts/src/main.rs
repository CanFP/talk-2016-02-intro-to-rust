enum Command {
    Download(String, String),
    Run(String),
    Sleep(i32),
    ChangeSetting(String, String),
    DumpSettings,
}

enum Direction {
    N(i32),
    E(i32),
    S(i32),
    W(i32),
    NE(i32, i32),
    SE(i32, i32),
    SW(i32, i32),
    NW(i32, i32),
}

fn main() {
    let walk_left = Direction::W(1);
    let run_up    = Direction::N(2);
    let x,y = 0;

    let movement = walk_left;

    match movement {
        Direction::N(n) => new_pos();
        Command::Download(web, path) => println!("Going to download {}", web),
        Command::Run(path)           => println!("Going to run {}", path),
        _                            => println!("Going to do .. something else")
    }
}

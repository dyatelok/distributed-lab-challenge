use maze::Maze;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let args = args.iter().map(|s| &**s).collect::<Vec<_>>();

    let (width, height) = match args[..] {
        [width, height] => {
            if let (Ok(width), Ok(height)) = (width.parse(), height.parse()) {
                (width, height)
            } else {
                panic!("Can't parse parameters as numbers. Use: `cargo run width height`")
            }
        }
        _ => panic!("Unsupported number of arguments. Use: `cargo run width height`"),
    };

    let maze = Maze::new(width, height, None);
    println!("{maze}");
}

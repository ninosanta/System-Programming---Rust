use minesweeper::annotate;
use std::env::args;

fn main() {
    let args: Vec<String> = args().skip(1).collect::<Vec<String>>();
    if args.len() != 3 {
        println!("Wrong number of args!");
        return;
    }
    let mut rows= 0;
    let mut cols = 0;

    println!("Args: {:?}", args);
    if args[0].contains("--rows=") {
        rows = args[0].replace("--rows=", "").parse::<usize>().unwrap();
    }
    if args[1].contains("--cols=") {
        cols = args[1].replace("--cols=", "").parse::<usize>().unwrap();
    }
    println!("rows: {}, cols: {}", rows, cols);

    let field =
        (0..rows)
            .map(|_i| {
                (0..cols)
                    .map(|j| (args[2].as_bytes()[j] as char))
                    .collect()
            })
            .collect::<Vec<String>>();
    let sliced_field = field
        .iter()
        .map(|s| &s[..])
        .collect::<Vec<_>>();

    println!("{:?}", field);

    let _test = annotate(&[
        " * * ",
        "  *  ",
        "  *  ",
        "     ",
    ]);

    println!("{:?}", annotate(&sliced_field));
}
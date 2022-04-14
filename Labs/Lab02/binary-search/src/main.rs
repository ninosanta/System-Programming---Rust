/**
 * Run, for example, as:
 *      cargo run -- 11 < input.txt
 */

use std::env::args;
use std::io::{self, Read};
use binary_search::{find, find_recur};
//use binary_search::find;

fn main() {
    let args: String = args().skip(1).collect::<String>();
    let key = args.parse::<i32>().expect("not a digit");

    let mut stdin = io::stdin();
    //let mut lines = stdin.lock().lines().map(|x| x.expect("eew")).collect::<Vec<&str>>();
    let mut lines = String::new();
    stdin.read_to_string(&mut lines)
        .expect("stdin not found");

    println!("{:?}", find(lines
                              .lines()  /* it splits by '\n' */
                              .map(|l| l.parse::<i32>().unwrap())
                              .collect::<Vec<i32>>(), key));
}

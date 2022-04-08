/**
 * Lab. #1 - Ersercizio 1
 */
use capitalize::capitalize;
use std::env::args;

fn main() {
    //let s: String = "  questa  è  una   frase ß  ".to_string();
    let args: String = args().skip(1).collect::<String>();
    let s1: String = capitalize(&args);

    println!("{}", s1);
}

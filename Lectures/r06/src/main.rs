/**
 * Lez. #6
 */

mod main_v1;
mod main_v2;
mod main_v3;

const VERSION: &str = "v3";

fn main() {
    match VERSION {
        "v1" => main_v1::main_v1(),
        "v2" => main_v2::main_v2(),
        "v3" => main_v3::main_v3(),
        _ => println!("unexpected version"),
    }
}

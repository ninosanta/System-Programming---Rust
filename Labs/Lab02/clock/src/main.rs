use clock::Clock;

fn main() {
    let t1: Clock = Clock::new(00, 25);
    let t2: Clock = Clock::new(00, 30);

    println!("t1: {}, t2: {}", t1, t2);
    println!("t1 - 255 minutes: {}", t1 - 255);
    println!("t1 + t2: {}", t1 + t2);
}
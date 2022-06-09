/** Look at slides 50 of l08 before! **/

use std::io::{BufRead, BufReader, Write};
use std::ops::Add;
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use crate::data::Point;


mod data {
    use std::fmt::Error;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize,Deserialize, Debug)]
    pub struct Point {
        lat: f64,
        lng: f64,
    }

    /* struct that will be exchanged */
    impl Point {
        pub fn new(lat: f64, lng: f64) -> Self {
            if (lat < -90.0 || lat > 90.0 || lng < -180.0 || lng > 180.0) {
                panic!("Invalid coordinates ({}, {})", lat,lng);
            }
            Point {
                lat,
                lng
            }
        }

        pub fn to_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }

        pub fn from_json(json: &str) -> Result<Point, serde_json::Error> {
            serde_json::from_str(json)
        }
    }

}

fn start_process(sender: Sender<String>, receiver: Receiver<String>) {

    let child = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start process");

    thread::spawn(move || {
        let mut pipe_in = BufReader::new(child.stdout.unwrap());
        let mut pipe_out = child.stdin.unwrap();
        for mut line in receiver {
            line.push('\n');
            pipe_out.write_all(line.as_bytes()).unwrap();
            pipe_out.flush();
            let mut buf = String::new();
            match pipe_in.read_line(&mut buf) {
                Ok(_) => {
                    sender.send(buf).unwrap();  // it forwards the output from channel 2 to ch1
                    continue;
                }
                Err(e) => {
                    println!("an error!: {:?}", e);
                    break;
                }
            }
        }
    });
}

/* it creates Points and sends the out to the Sender in a JSON format */
fn start_command_thread(sender: Sender<String>) {
    thread::spawn(move || {
        for i in 1..10 {
            let n = i as f64;
            let p = Point::new(n*10.0, n*20.0);
            sender.send(String::from( p.to_json())).unwrap();
        }
    });
}

fn main() {
    /* channels pair */
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();

    start_process(tx1, rx2);

    start_command_thread(tx2);

    /* read from the return channel */
    rx1.iter().for_each(|line| {
        let line = line.replace("\n","");  // \n is thrown away to decoding
        let p = Point::from_json(&line).unwrap();  // decoding from Point to JSON
        println!("Received {:?}", p);
    } );
}
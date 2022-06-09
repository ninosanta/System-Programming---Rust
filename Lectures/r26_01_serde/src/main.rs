use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Command {
    cmd: String,
    args: Vec<i32>,
    flag: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "tag", content = "data")]  // to have an enum easier to read and parse
enum Command1 {
    Forward(f64),
    Right(f64),
    Left(f64),
    Stop,
    //Jump(i32, String),
    Jump(i32, Vec<Option<i32>>),
}

impl Command {
    fn new(cmd: String, args: Vec<i32>, flag: bool) -> Self {
        Command {
            cmd, args, flag,
        }
    }

    fn to_json(&self) -> String {
        /* it transform the struct into a Json */
        serde_json::to_string(self).unwrap()
    }

    fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}


impl Command1 {
    fn to_json(&self) -> String {
        /* it transform the struct into a Json */
        serde_json::to_string(self).unwrap()
    }

    fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

fn main() {
    let cmd1: Command = Command::new("alpha".to_string(), vec![1,2,3,4], false);
    let s = cmd1.to_json();
    println!("{:?} => JSON {}", cmd1, s);
    match Command::from_json(&s) {
        Ok(cmd2) => println!("JSON {} => {:?}", s, cmd2),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("==============================");
    for cmd2 in [
        Command1::Forward(5.0),
        //Command1::Jump(32, "hello".to_string()),
        Command1::Jump(32, vec![Some(3), Some(8), None, Some(-43)]),
        Command1::Stop,
        Command1::Left(-32.4),
    ] {
        let s = cmd2.to_json();
        println!("{:?} => JSON {}", cmd2, s);
        match Command1::from_json(&s) {
            Ok(cmd3) => println!("JSON {} => {:?}", s, cmd3),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

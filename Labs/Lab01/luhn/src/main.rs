use std::env::args;
use luhn::is_valid;

fn main() {
    let mut verify: bool = true;
    let args: Vec<String> = args().skip(1).collect::<Vec<String>>();
    if args.len() == 0 {
        panic!("There are no args!");
    }

    /* for each string wrote in the command line */
    for arg in args {
        /* Vec of groups of 4 digits */
        let groups: Vec<&str> = arg.split(' ')
            .filter(|s| { !s.is_empty() })  /* "8273 1232 7352 " */
            .collect::<Vec<&str>>();
        
        if groups.len() != 4 {
            print!("{} is not a valid number! ", arg);
            println!("Error: less or more than 4 groups");
            continue;
        }
        
        for group in groups {
            if group.chars().count() != 4 ||
                !match group.parse::<u32>() { Ok(_n) => true, Err(_n) => false, }
            {
                print!("{} is not a valid number! ", arg);
                println!("Error: less or more than 4 digits in one group,\
                 or it was not a number at all");
                verify = false;
                break;
            }
        }

        if verify == true {
            if is_valid(&arg) {
                println!("{} is a valid number!", arg);
            } else {
                println!("{} is not a valid number!", arg);
            }
        } else {
            verify = true;
        }
    }
    return;
}

/*
    This check does not work because if a group starts with 0 e.g., 0956 it is parsed to 956 as u32
    so that it will be not valid even if it is.
        match str.parse::<u32>() {
            Ok(n) => {
                if n < 1000 || n > 9999 {
                    println!("Less than 4 digits in one group: {}!", n);
                    break;
                }
            },
            Err(n) => {
                println!("{} is not a number!", n);
                break;
            }
        }
 */
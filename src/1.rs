use std::{env, fs, process};

struct Config {
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    let res: i32 = make_integer_list(contents).iter().sum();

    println!("{}", res);
}

fn make_integer_list(contents: String) -> Vec<i32> {
    let mut ret: Vec<i32> = Vec::new();

    for l in contents.lines() {
        let mut first: char = 'x';
        let mut last: char = 'x';

        for c in l.chars() {
            if c.is_ascii_digit() {
                if first.eq(&'x') {
                    first = c;
                    last = c;
                } else {
                    last = c;
                }
            }
        }
        ret.push(format!("{}{}", first, last).parse::<i32>().unwrap());
    }
    return ret;
}

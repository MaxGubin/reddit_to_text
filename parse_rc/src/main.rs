use std::fs::File;
use std::io::{self, BufRead, Write, Error};
use std::path::Path;

extern crate rustc_serialize;
use rustc_serialize::json::Json;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
        let file = File::open(filename)?;
            Ok(io::BufReader::new(file).lines())
}


fn process_file<P>(input_filename: P, output_filename: P) -> Result<(), Error>  
where P: AsRef<Path>, {
    let mut output = File::create(output_filename)?;
    if let Ok(lines) = read_lines(input_filename) {
        for line in lines {
            if let Ok(js) = line {
                let json = Json::from_str(&js).unwrap();
                let body = json["body"].as_string().unwrap_or("");
                let trim_body = body.trim().replace("\n", " ");
                if !trim_body.is_empty() && trim_body.chars().next().unwrap() != '[' {
                    write!(output, "{}\n", trim_body);
                }
            }
        }
    }
    Ok(())
}

fn main()  -> Result<(), Error> {
    process_file(Path::new("../rc_2019.json"), 
                 Path::new("../rc_2019.txt"))
}


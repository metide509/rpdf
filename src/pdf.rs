use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::trailer::{Info, Root, Trailer};
pub struct PDF {
    path: String,
    trailer: Trailer,
}

impl From<&str> for PDF {
    fn from(path: &str) -> Self {
        //let file = "C:\\Users\\metide\\Desktop\\１６542大阳四轮车.pdf";
        let input: File = File::open(path).unwrap();
        let buffered: BufReader<File> = BufReader::new(input);

        let mut is_trailer = false;

        let mut trailer = Trailer::new();

        for line in buffered.lines() {
            match line {
                Ok(line) => {
                    if is_trailer {
                        if line.starts_with("<<") {
                            println!("begin read tariler ...");
                        } else if line.starts_with(">>") {
                            println!("end read tariler ...");
                            is_trailer = false;
                        } else {
                            println!("{}", line);
                            let ps: Vec<String> =
                                line.split(" ").map(|x| String::from(x)).collect();
                            let first = ps.get(0).unwrap().as_str();

                            println!("{:?}", &ps);

                            match first {
                                "/Info" => trailer.set_info(Info {}),
                                "/Root" => trailer.set_root(Root {}),
                                "/Size" => trailer.set_size(ps.get(1).unwrap().parse().unwrap()),
                                _ => println!("{}", first),
                            }
                        }
                    }

                    if line.starts_with("trailer") {
                        is_trailer = true;
                    }
                }
                Err(e) => {}
            }
        }

        println!("{:?}", trailer);

        PDF {
            path: String::from(path),
            trailer,
        }
    }
}

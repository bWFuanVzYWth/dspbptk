// use std::env;
// use std::fs;

mod blueprint;
mod md5;

fn main() {
    use blueprint::blueprint;
    use md5::{Algo::MD5F, MD5};
    use nom::Finish;

    let string = "BLUEPRINT:0,0,0,0,0,0,0,0,0,0.0.0.0,,\"H4sIAAAAAAAAA2NkQAWMUMyARCMBANjTKTsvAAAA\"E4E5A1CF28F1EC611E33498CBD0DF02B\n\0";
    let result = blueprint::parser(string);
    let test = String::from(result.head);
    let hash = MD5::new(MD5F).process((test + "\"" + result.data).as_bytes());
    let hex_string: String = hash
        .iter()
        .map(|&byte| format!("{:02X}", byte))
        .collect::<Vec<_>>()
        .join("");
    println!("{hex_string}");
}

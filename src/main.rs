use serde_json;
use std::fs;
use std::env;

fn main() {
    let path_buf = home::home_dir().unwrap();
    let path = path_buf.into_os_string().into_string().unwrap();
    //println!("{}", path);

    let mut home_path: String = path.to_owned();
    let wal_json: &str = "/.cache/wal/colors.json";
    home_path.push_str(wal_json);
    //println!("{}", home_path);

    let json_path = home_path;
    let data = fs::read_to_string(json_path).expect("Unable to read file");
    println!("{}", data);
    




}
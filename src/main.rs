use serde_json;
use serde_json::{Map, Value};
use std::fs;
use std::env;
use serde_json::{Result};
use serde::{Deserialize, Serialize};

fn main() {
    untyped_example();
}

#[derive(Serialize, Deserialize)]
struct WalCache {
    wallpaper: String,
    alpha: String,
    special: Vec<String>,
    colors: Vec<String>,
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn untyped_example() -> Result<()> {
    let path_buf = home::home_dir().unwrap();
    let path = path_buf.into_os_string().into_string().unwrap();

    let mut home_path: String = path.to_owned();
    let wal_json: &str = "/.cache/wal/colors.json";
    home_path.push_str(wal_json);

    let json_path = home_path;
    let data = fs::read_to_string(json_path).expect("Unable to read file");
    print_type_of(&data);

    //let v: Value = serde_json::from_str(json_path)?;

    //println!("Please call {} at the number", v["wallpaper"]);
    //println!("{}", &data);

    //let w: WalCache = serde_json::from_str(&data)?;
    println!("{}", data);

    Ok(())
}

    
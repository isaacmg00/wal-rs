use serde_json::{Value, Result};
use std::fs;
use colored::*;
use css_color_parser::Color as CssColor;
use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    untyped_example();

    let selections = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor")
        .default(1)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("Enjoy your {}!", selections[selection]);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

fn print_colors(v:&Value, len: usize) -> Vec<String> {
    let path_buf = home::home_dir().unwrap();
    let path = path_buf.into_os_string().into_string().unwrap();

    let mut home_path: String = path.to_owned();
    let wal_json: &str = "/.cache/wal/colors.json";
    home_path.push_str(wal_json);

    let json_path = home_path;
    let data = fs::read_to_string(json_path).expect("Unable to read file");

    const EMPTY_STRING: String = String::new();
    let mut wal_colors: [String; 16] = [EMPTY_STRING; 16];

    let mut vec = Vec::with_capacity(len);

    vec.push(v["colors"]["color0"].to_string());
    vec.push(v["colors"]["color1"].to_string());
    vec.push(v["colors"]["color2"].to_string());
    vec.push(v["colors"]["color3"].to_string());
    vec.push(v["colors"]["color4"].to_string());
    vec.push(v["colors"]["color5"].to_string());
    vec.push(v["colors"]["color6"].to_string());
    vec.push(v["colors"]["color7"].to_string());
    vec.push(v["colors"]["color8"].to_string());
    vec.push(v["colors"]["color9"].to_string());
    vec.push(v["colors"]["color10"].to_string());
    vec.push(v["colors"]["color11"].to_string());
    vec.push(v["colors"]["color12"].to_string());
    vec.push(v["colors"]["color13"].to_string());
    vec.push(v["colors"]["color14"].to_string());
    vec.push(v["colors"]["color15"].to_string());


    for i in 0..len {
        let transparent_black = CssColor { r: 0, g: 0, b: 0, a: 1.0 };

        vec[i] = rem_first_and_last(&vec[i]).to_string();
        let rgb_color = vec[i].parse::<CssColor>().unwrap_or(transparent_black);
        let mut red = rgb_color.r;
        let mut green = rgb_color.g;
        let mut blue = rgb_color.b;

        println!("{}", vec[i].on_truecolor(red, green, blue));
    }

    let selections = &[
        "Ice Cream".on_truecolor(255,255,255),
        "Vanilla Cupcake".on_truecolor(255,255,255),
        "Chocolate Muffin".on_truecolor(255,255,255),
        "A Pile of sweet, sweet mustard".on_truecolor(255,255,255),
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor")
        .default(1)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("Enjoy your {}!", selections[selection]);

    return vec;

}
fn untyped_example() -> Result<()> {
    let path_buf = home::home_dir().unwrap();
    let path = path_buf.into_os_string().into_string().unwrap();

    let mut home_path: String = path.to_owned();
    let wal_json: &str = "/.cache/wal/colors.json";
    home_path.push_str(wal_json);

    let json_path = home_path;
    let data = fs::read_to_string(json_path).expect("Unable to read file");

    let v: Value = serde_json::from_str(&data)?;
    
    let generated_vec = print_colors(&v, 16);
    //println!("{:?}", generated_vec);


    const EMPTY_STRING: String = String::new();
    let mut wal_colors: [String; 16] = [EMPTY_STRING; 16];

    wal_colors[0]=v["colors"]["color0"].to_string();
    wal_colors[1]=v["colors"]["color1"].to_string();
    wal_colors[2]=v["colors"]["color2"].to_string();
    wal_colors[3]=v["colors"]["color3"].to_string();
    wal_colors[4]=v["colors"]["color4"].to_string();
    wal_colors[5]=v["colors"]["color5"].to_string();
    wal_colors[6]=v["colors"]["color6"].to_string();
    wal_colors[7]=v["colors"]["color7"].to_string();
    wal_colors[8]=v["colors"]["color8"].to_string();
    wal_colors[9]=v["colors"]["color9"].to_string();
    wal_colors[10]=v["colors"]["color10"].to_string();
    wal_colors[11]=v["colors"]["color11"].to_string();
    wal_colors[12]=v["colors"]["color12"].to_string();
    wal_colors[13]=v["colors"]["color13"].to_string();
    wal_colors[14]=v["colors"]["color14"].to_string();
    wal_colors[15]=v["colors"]["color15"].to_string();
    Ok(())
}

    
use serde_json::{Value, Result};
use std::fs;
use colored::*;
use css_color_parser::Color as CssColor;
use dialoguer::{theme::ColorfulTheme, Select};
use std::process::Command;
use std::fmt::Display;
use std::sync::Arc;

fn main() {
    println!("{}", get_device_name());
    wal_rs().ok();
}

fn get_device_name() -> String {
    let output = Command::new("ratbagctl")
                     .arg("list")
                     .output()
                     .expect("failed to execute process");

    let ratbagctl_output = String::from_utf8_lossy(&output.stdout);

    let device_name: Vec<String> = ratbagctl_output.split(":").map(|s| s.to_string()).collect();
    return device_name[0].to_string();
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

    let mut vec = Vec::with_capacity(len);
    let mut r = Vec::with_capacity(len);
    let mut g = Vec::with_capacity(len);
    let mut b = Vec::with_capacity(len);

    home_path.push_str(wal_json);

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
        let red = rgb_color.r;
        let green = rgb_color.g;
        let blue = rgb_color.b;
        r.push(red);
        g.push(green);
        b.push(blue);
    }

    let selections = &[
        vec[0].bold().on_truecolor(r[0],g[0],b[0]),
        vec[1].bold().on_truecolor(r[1],g[1],b[1]),
        vec[2].bold().on_truecolor(r[2],g[2],b[2]),
        vec[3].bold().on_truecolor(r[3],g[3],b[3]),
        vec[4].bold().on_truecolor(r[4],g[4],b[4]),
        vec[5].bold().on_truecolor(r[5],g[5],b[5]),
        vec[6].bold().on_truecolor(r[6],g[6],b[6]),
        vec[7].bold().on_truecolor(r[7],g[7],b[7]),
        vec[8].bold().on_truecolor(r[8],g[8],b[8]),
        vec[9].bold().on_truecolor(r[9],g[9],b[9]),
        vec[10].bold().on_truecolor(r[10],g[10],b[10]),
        vec[11].bold().on_truecolor(r[11],g[11],b[11]),
        vec[12].bold().on_truecolor(r[12],g[12],b[12]),
        vec[13].bold().on_truecolor(r[13],g[13],b[13]),
        vec[14].bold().on_truecolor(r[14],g[14],b[14]),
        vec[15].bold().on_truecolor(r[15],g[15],b[15]),
    ];

    let mut selection_index = 0;
    println!("{}", "Pywal Theme Colors");

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("colors:")
            .default(selection_index)
            .items(&selections[..])
            .interact()
            .unwrap();

        selection_index = selection;

        println!("{}", "Setting Peripheral RGB Color...");
        let mut hex_color: String = vec[selection].to_string();
        hex_color.remove(0);

        let arc = Arc::new(hex_color);
        let mut hex_code = vec![];

        hex_code.push(arc.clone());
        let mut elements2 = vec![];
        elements2.push(arc.clone());

        let mouse_command = Command::new("ratbagctl")
                            .arg(get_device_name())
                            .arg("led")
                            .arg("0")
                            .arg("set")
                            .arg("color")
                            .arg(hex_code[0].to_string())
                            .output()
                            .expect("failed to execute process");   
  
        let kb_command = Command::new("g810-led")
                            .arg("-fx")
                            .arg("color")
                            .arg("keys")
                            .arg(hex_code[0].to_string())
                            .output()
                            .expect("failed to execute process"); 
                            
        let kb_logo_command = Command::new("g810-led")
                            .arg("-fx")
                            .arg("color")
                            .arg("logo")
                            .arg(hex_code[0].to_string())
                            .output()
                            .expect("failed to execute process");    
        
    }
    return vec;

}

fn wal_rs() -> Result<()> {    
    let path_buf = home::home_dir().unwrap();
    let path = path_buf.into_os_string().into_string().unwrap();
    let mut home_path: String = path.to_owned();
    let wal_json: &str = "/.cache/wal/colors.json";
    home_path.push_str(wal_json);
    let json_path = home_path;
    let data = fs::read_to_string(json_path).expect("Unable to read file");
    let v: Value = serde_json::from_str(&data)?;
    print_colors(&v, 16);
    Ok(())
}

    
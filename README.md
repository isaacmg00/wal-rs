# wal-rs
wal-rs is a rust CLI I made to keep a consistent theme with both my desktop wallpaper and RGB peripherals (mouse and keyboard). It searches for a JSON file at ```~/.cache/wal/colors.json``` to load 16 colors generated by pywal that you can choose from to set the KBM RGB color.

<h3 align="center"><img src="https://i.imgur.com/gverfd7.gif" width="760"></h3>

**Note:** This script has only been tested using Logitech devices on Linux. For Windows use the G Hub software.

### Dependencies
  - pywal
  - g810-led
  - ratbagctl

### Working Devices

| Mouse | Keyboard |
| --------- | ------ |
| G PRO Wireless | G PRO |

### Usage
Skip this step if you already have rust installed.
```sh
curl https://sh.rustup.rs -sSf | sh  # install rust with cURL
```
In a terminal, run
```sh
git clone https://github.com/isaacmg00/wal-rs
cd wal-rs/
cargo run
```

Enjoy!




**TODO:** error handling, refactoring, progress bars for looks, expand functionality for more Logitech devices.

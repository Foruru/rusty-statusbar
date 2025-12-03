# Rusty status bar
Simple status bar for dwm written in rust. Easy to use.

## What it can show
 - Time
 - Date
 - CPU temperature
 - Keyboard layout

## Usage

### Dependencies
 - rust
 - cargo
 - libxcb1-dev

### Installation
```
cd rusty-statusbar
cargo install --path .
```
and then put it into your dwm start script
```
rusty-statusbar --loop &
```

### Command-line options
```
Usage: rusty-statusbar [OPTIONS]

Options:
    -h, --help
        Display this help message and exit.
    -r, --refresh-rate <time>
        Set refresh rate in milliseconds, value must be an integer.
    -l, --loop
        Loop program.
```

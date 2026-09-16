use console::style;
use std::{thread, time::Duration};

fn main() {
    println!("Welcome to {}! This is a simple CLI tool to configurate your fastfetch settings.", style("effc").magenta());
    thread::sleep(Duration::from_millis(1000));
    println!("What would you like to edit?");
    // TODO: prompt the user
    
}
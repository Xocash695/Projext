#![allow(unused)]
// witten in rust programming language
use std::process::Command;
extern crate cfonts;
use cfonts::{ say, Options, Fonts };
use tts_rust::{ tts::GTTSClient, languages::Languages };
fn main() {
    println!("PROGRAM STARTING UP");
    println!("Built by Xocash695");
    
    if let Some((w, h)) = term_size::dimensions() {
        println!("Width: {}\nHeight: {}", w, h);
        println!("optimzing for terminal size");
      } else {
        println!("Unable to get term size :(");
        println!("Terminal size is unoptimized");
    }
    let mut child = Command::new("sleep").arg("0.8").spawn().unwrap();
    let result = child.wait().unwrap();
    print!("\x1B[2J\x1B[1;1H");
    
    for i in 1..6{
        for j in 1..20{
            print!(" . ");
            if j % 2 == 0 {
                print!("*");
            } else{
                print!("@");
            }
            let mut child = Command::new("sleep").arg("0.01").spawn().unwrap();
            let result = child.wait().unwrap();
        }
        println!("");
    }
    say(Options {
        text: String::from("Welcome to Projext"),
        font: Fonts::FontTiny,
        ..Options::default()
    });
    let mut narrator: GTTSClient = GTTSClient {
        volume: 1.0, 
        language: Languages::English, // use the Languages enum
        tld: "com",
    };
    narrator.speak("Welcome to Projext");

}


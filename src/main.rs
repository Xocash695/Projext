#![allow(unused)]
// witten in rust programming language
use std::process::{Command, Child, ExitStatus};
extern crate cfonts;
use cfonts::{ say, Options, Fonts };
use tts_rust::{ tts::GTTSClient, languages::Languages };
use indicatif::ProgressBar;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};

fn main()  {
    let mut child;
    let mut result; 
    boot();
    logo();
    child = Command::new("sleep").arg("0.4").spawn().unwrap();
    result = child.wait().unwrap();
    println!("What would you like to do?");

}

fn boot() {
    let mut child:Child;
    let mut result:ExitStatus; 
    println!("PROGRAM STARTING UP");
    let bar = ProgressBar::new(1000);
     for _ in 0..1000 {
         bar.inc(1);
         child = Command::new("sleep").arg("0.01").spawn().unwrap();
         result = child.wait().unwrap();
         // ...
     }
     bar.finish();
    child = Command::new("sleep").arg("0.4").spawn().unwrap();
    result = child.wait().unwrap();
    println!("Built by Xocash695");
    
    if let Some((w, h)) = term_size::dimensions() {
        println!("Width: {}\nHeight: {}", w, h);
        child = Command::new("sleep").arg("0.4").spawn().unwrap();
        result = child.wait().unwrap();
        println!("optimzing for terminal size");
      } else {
        println!("Unable to get term size :(");
        child = Command::new("sleep").arg("0.4").spawn().unwrap();
        result = child.wait().unwrap();
        println!("Terminal size is unoptimized");
    }
    child = Command::new("sleep").arg("0.4").spawn().unwrap();
    result = child.wait().unwrap();
    print!("\x1B[2J\x1B[1;1H");
}

fn logo() {
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
fn selector() -> std::io::Result<()>{
    let items = vec!["Launch android phone (Requires android studio)", "Launch Iphone phone (Requires system running MacOS and Xcode)", "Make website (Requires Docker)", "Make AI"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => println!("User selected item : {}", items[index]),
        None => println!("User did not select anything")
    }

    Ok(())
}
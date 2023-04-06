#![allow(unused)]
// witten in rust programming language
use std::process::{Command, Child, ExitStatus, Stdio};
extern crate cfonts;
use cfonts::{ say, Options, Fonts };
use tts_rust::{ tts::GTTSClient, languages::Languages };
use indicatif::ProgressBar;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use std::env;
use std::fs;
use execute::{Execute, shell};
use dialoguer::Confirm;

fn main()  {
    let mut child;
    let mut result; 
    boot();
    logo();
    child = Command::new("sleep").arg("0.4").spawn().unwrap();
    result = child.wait().unwrap();
    selector();
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to continue using selector or end program?")
        .interact()
        .unwrap()
    {
        println!("Looks like you want to continue using selector");
        selector();
        goodbye()

    } else {
        println!("alright then :(");
        goodbye();
    }
    
}

fn boot() {
    print!("\x1B[2J\x1B[1;1H");
    let mut child:Child;
    let mut result:ExitStatus; 
    println!("PROGRAM STARTING UP");
    let bar = ProgressBar::new(100);
     for _ in 0..100 {
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
      } else {
        println!("Unable to get term size :(");
        child = Command::new("sleep").arg("0.4").spawn().unwrap();
        result = child.wait().unwrap();
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
fn selector() {
    let mut narrator: GTTSClient = GTTSClient {
        volume: 1.0, 
        language: Languages::English, // use the Languages enum
        tld: "com",
    };
    narrator.speak("what do you want to do");
    narrator.speak("Options are listed below");
    let selections = &[
        "Launch Iphone",
        "Launch Android phone",
        "show Feet pic",
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you want to do?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();
    println!("Okay  {}!", selections[selection]);
    let message = "Okay doing";
    let sayresult = [message, selections[selection]].join("\n");
    narrator.speak(&sayresult);
    match selection {
        0 => iphone(),
        1 => android(),
        2 => feet(),
        _ => print!("You didn't select anything"),
    }

}

fn feet() { 
    let filepath = "feet.txt";
    let contents = fs::read_to_string(filepath)
    .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

fn android() {
    let mut command;

    command = shell("cd ~/Library/Android/sdk/emulator");
    
    command.stdout(Stdio::piped());

    command.execute_output().unwrap();

    command = shell("./emulator -avd Pixel_6_Pro_API_UpsideDownCake");
    
    command.stdout(Stdio::piped());

    command.execute_output().unwrap();

}

fn iphone() {
    let mut command;

    command = shell("open -a Simulator --args -CurrentDeviceUDID 936604F6-449D-4373-B8AD-94C7D08A7777");
    
    command.stdout(Stdio::piped());

    command.execute_output().unwrap();

}
fn goodbye() {
    say(Options {
        text: String::from("Thank you and Goodbye"),
        font: Fonts::FontTiny,
        ..Options::default()
    });
    let mut narrator: GTTSClient = GTTSClient {
        volume: 1.0, 
        language: Languages::English, // use the Languages enum
        tld: "com",
    };
    narrator.speak("Thank you and Goodbye");
}
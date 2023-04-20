#![allow(unused)]
// witten in rust programming language
use std::process::{Child, Command, ExitStatus, Stdio};
extern crate cfonts;
use cfonts::{say, Fonts, Options};
use dialoguer::Confirm;
use dialoguer::Input;
use dialoguer::{ theme::ColorfulTheme, Select};
use execute::{shell, Execute};
use indicatif::ProgressBar;
use tts_rust::{languages::Languages, tts::GTTSClient};
static mut NAME: &'static str = "";



fn main() {
    let mut child;
    let mut result;
    boot();
    logo();
    child = Command::new("sleep").arg("0.4").spawn().unwrap();
    result = child.wait().unwrap();
    let input: String = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Your name")
    .default("Master".to_string())
    .interact_text()
    .unwrap();
    unsafe {NAME = Box::leak(input.into_boxed_str());}
    selector();
    while Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to continue using selector?")
        .interact()
        .unwrap()
    {
        println!("Looks like you want to continue using selector");
        selector();
    }
    goodbye()
}


fn boot() {
    print!("\x1B[2J\x1B[1;1H");
    let mut child: Child;
    let mut result: ExitStatus;
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
    for i in 1..6 {
        for j in 1..20 {
            print!(" . ");
            if j % 2 == 0 {
                print!("*");
            } else {
                print!("@");
            }
            let mut child = Command::new("sleep").arg("0.01").spawn().unwrap();
            let result = child.wait().unwrap();
        }
        println!("");
    }
    let mut narrator: GTTSClient = GTTSClient {
        volume: 1.0,
        language: Languages::English, // use the Languages enum
        tld: "com",
    };
    let message = "Welcome to Projext ";
    narrator.speak(message);
    say(Options {
        text: String::from(message),
        font: Fonts::FontTiny,
        ..Options::default()
    });
 
   
  
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
        "show Feet pic",
        "System Info ",
        "Launch Android phone",
        "Launch Iphone",
        "Wifi speed test",
        "The matrix",
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
        0 => finselct(0),
        1 => finselct(1),
        2 => finselct(2),
        3 => finselct(3),
        4 => finselct(4),
        5 => finselct(5),
        _ => print!("You didn't select anything"),
    }
}

fn finselct(num: usize) {
    let sentance = [
        "you bad boy you shouldn't be looking at these things",
        " I think your already aware of the hardware this computer is running, but okay",
        "you already have a phone you don't need another but okay",
        "you already have a phone you don't need another but okay",
        "I think I'm faster than your wifi",
        "I'm hacking into the matrix for you",
    ];
    let commands = [
        "cd scripts && sh feet.sh",
        "cd scripts && sh neofetch.sh",
        "cd scripts && sh android.sh",
        "open -a Simulator --args -CurrentDeviceUDID 936604F6-449D-4373-B8AD-94C7D08A7777",
        "cd scripts && sh speedtest.sh",
        "cd scripts && sh matrix.sh",
    ];
    let mut narrator: GTTSClient = GTTSClient {
        volume: 1.0,
        language: Languages::English, // use the Languages enum
        tld: "com",
    };
    unsafe{ 
        let sayresult = [NAME, sentance[num]].join("\n");
        println!("{}", sayresult);
        narrator.speak(&sayresult);
    }

    let mut command;

    command = shell(commands[num]);

    command.stdout(Stdio::piped());

    command.execute_output().unwrap();
}

fn goodbye() {
    let mut narrator: GTTSClient = GTTSClient {
        volume: 1.0,
        language: Languages::English, // use the Languages enum
        tld: "com",
    };
    let message = "Thank you and Goodbye";
    narrator.speak(message);
    say(Options {
        text: String::from(message),
        font: Fonts::FontTiny,
        ..Options::default()
    });
}
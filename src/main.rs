#![allow(unused)]
#[cfg(not(target_os = "macos"))]
compile_error!("only macOS is supported");
// written in rust programming language
use std::process::{Child, Command, ExitStatus, Stdio};
extern crate cfonts;
use cfonts::{say, Fonts, Options};
use chrono::{ Local, Timelike};
use colored::Colorize;
use dialoguer::Confirm;
use dialoguer::Input;
use dialoguer::{theme::ColorfulTheme, Select};
use execute::{shell, Execute};
use indicatif::ProgressBar;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::time::{Instant};
use tts_rust::{languages::Languages, tts::GTTSClient};
static mut NAME: &'static str = "";

fn main() {
    let start = Instant::now(); //starts stopwatch
    let mut child;
    let mut result;
    boot(); // shows progress bar along with date and time
    logo(); // show welcome projext text
    let duration = start.elapsed(); // end stopwatch
    println!("{} {:?}", "BOOT DURATION".italic().blue(), duration); // prints how long it took to do the functions
    name(); // grabs name from user
    child = Command::new("sleep").arg("0.4").spawn().unwrap(); // waits 0.4 seconds
    result = child.wait().unwrap();
    selector(); // asks user to select option
    while Confirm::with_theme(&ColorfulTheme::default()) // repeats loop until user wants to stop
        .with_prompt("Do you want to continue using selector?")
        .interact()
        .unwrap()
    {
        println!("Looks like you want to continue using selector");
        selector();
    }
    goodbye() // goodbye message
}

fn boot() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("/Users/akash/Source/projext/music/server-startup-41826.mp3").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples());
    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(1));
    let now = Local::now(); // Grabs local time
    let (is_pm, hour) = now.hour12();
    print!("\x1B[2J\x1B[1;1H"); // clears console
    let mut child: Child;
    let mut result: ExitStatus;
    println!("{}", "PROGRAM STARTING UP".green().italic());
    println!(
        "{} {:02}:{:02}:{:02} {}",
        "DATE".italic().red(),
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    ); // prints time
    let bar = ProgressBar::new(100); // progress bar length can be controled by changing value
    for _ in 0..100 {
        bar.inc(1);
        child = Command::new("sleep").arg("0.01").spawn().unwrap();
        result = child.wait().unwrap();
        // ...
    }
    bar.finish();
    child = Command::new("sleep").arg("0.4").spawn().unwrap();
    result = child.wait().unwrap();

    println!("{}","Built by Xocash695".yellow());

    if let Some((w, h)) = term_size::dimensions() {
        //grabs terminal size and print out or if can't prints unable to
        println!("Terminal: Width: {} Height: {}", w, h);
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

fn name() {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your name")
        .default("Master".to_string())
        .interact_text()
        .unwrap();
    unsafe {
        NAME = Box::leak(input.into_boxed_str()); // sets global variable name to input
        if NAME == "akash" {
            // check if name is akash
            let mut narrator: GTTSClient = GTTSClient {
                volume: 1.0,
                language: Languages::English, // use the Languages enum
                tld: "com",
            };
            let creator = [
                "What! are you the creator, nooo way!",
                "Akash I missed you so much",
                "I thought you abandoned me",
                "Thank you for coming back",
            ];
            println!("");
            for i in 0..creator.len() {
                println!("{}", creator[i]);
                narrator.speak(creator[i]);
            }
        }
    }
}

fn logo() {
    for i in 1..6 {
        for j in 1..20 {
            print!(" . ");
            if j % 2 == 0 {
                print!("*");
            } else {
                print!("{}", "@".purple());
            }
            let mut child = Command::new("sleep").arg("0.01").spawn().unwrap();
            let result = child.wait().unwrap();
        }
        println!("");
    }
    let message = "Welcome to Projext";
    say(Options {
        text: String::from(message),
        font: Fonts::FontTiny,
        ..Options::default()
    });
    let mut narrator: GTTSClient = GTTSClient {
        volume: 1.0,
        language: Languages::English, // use the Languages enum
        tld: "com",
    };
    narrator.speak(message);
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
        "Show picture of creator",
        "System Info ",
        "Launch Android phone",
        "Launch Iphone",
        "Wifi speed test",
        "The matrix",
        "Play Nah by Akash Kallumkal",
        "Shutdown this computer",
        "AI Art generator",
        "Creator's github",
        "Resource monitor",
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
        // match case is similar to switch case in other programming languages
        0 => finselct(0),
        1 => finselct(1),
        2 => finselct(2),
        3 => finselct(3),
        4 => finselct(4),
        5 => finselct(5),
        6 => finselct(6),
        7 => finselct(7),
        8 => finselct(8),
        9 => finselct(9),
        10 => finselct(10),
        _ => print!("You didn't select anything"),
    }
}

fn finselct(num: usize) {
    let sentance = [
        "The creator of this project looks amazing as he always does",
        "I think your already aware of the hardware this computer is running, but okay",
        "you already have a phone you don't need another but okay",
        "you already have a phone you don't need another but okay",
        "I think I'm faster than your wifi",
        "I'm hacking into the matrix for you",
        "Your sure to love Nah by Akash Kallumkal",
        "Computer will shutdown in two minutes, please enter your password into terminal that pops up",
        "Have fun generating some AI Art",
        "The creator has some interesting projects",
        "The system monitor is pretty cool",
    ];
    let commands = [
        "cd ~/Source/projext/scripts && sh author.sh",
        "cd ~/Source/projext/scripts && sh neofetch.sh",
        "cd ~/Source/projext/scripts && sh android.sh",
        "open -a Simulator --args -CurrentDeviceUDID 936604F6-449D-4373-B8AD-94C7D08A7777",
        "cd ~/Source/projext/scripts && sh speedtest.sh",
        "cd ~/Source/projext/scripts && sh matrix.sh",
        "cd  ~/Source/projext/scripts && sh rap.sh",
        "cd  ~/Source/projext/scripts && sh shutdown.sh",
        "open /Applications/DiffusionBee.app",
        "open https://github.com/Xocash695",
        "open /System/Applications/Utilities/'Activity Monitor'.app",
    ];
    let mut narrator: GTTSClient = GTTSClient {
        volume: 1.0,
        language: Languages::English, // use the Languages enum
        tld: "com",
    };
    unsafe {
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
    let message = "Thank you and Goodbye";
    say(Options {
        text: String::from(message),
        font: Fonts::FontTiny,
        ..Options::default()
    });
    let mut narrator: GTTSClient = GTTSClient {
        volume: 1.0,
        language: Languages::English, // use the Languages enum
        tld: "com",
    };
    narrator.speak(message);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("/Users/akash/Source/projext/music/system-shutdown-63911.mp3").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples());
    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("SHUTDOWN: {}", "OK".green())
}

use beep_music::*;

use std::io;
use std::io::Read;

use clap::{Arg, App};

fn main() {

    let matches = App::new("BEEP Music")
        .version("0.1.0")
        .about("Plays music through the PC Speaker through custom syntax")
        .arg(Arg::with_name("bpm")
            .short("b")
            .long("bpm")
            .takes_value(true)
            .help("Beats-per-minute of the song (Default: 120)"))
        .arg(Arg::with_name("grub-tune")
            .short("g")
            .long("grub-tune")
            .takes_value(false)
            .help("Instead of playing, convert the input into a GRUB init tune"))
        .get_matches();

    let bpm;

    if let Ok(bpm_arg) = matches.value_of("bpm").unwrap_or("120").parse::<u16>() {
        bpm = bpm_arg;
    } else {
        eprintln!("BPM argument is not a valid number");
        return;
    }

    let grub_init_tune = matches.is_present("grub-tune");

    
    let input = read_stdin();

    if let Some(notes) = parse_notes(&input) {
        if grub_init_tune {
            create_grub_init_tune(notes, bpm);
        } else {
            play(notes, bpm);
        }
    } else {
        eprintln!("Syntax error");
    }
}

fn read_stdin() -> String {
    let stdin = io::stdin();
    let mut buf = String::new();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buf).expect("Oh no :(");
    }

    buf
}

fn play(notes: Vec<Note>, bpm: u16) {
    for note in notes {
        note.play(bpm);
    }
}

fn create_grub_init_tune(notes: Vec<Note>, bpm: u16) {
    print!("{}", bpm);
    for note in notes {
        print!(" {} {}",
            note.get_frequency() * 3, // approximation, grub tempo is weird
            
            5 - (if note.get_length() > 4 {
                4
            } else {
                note.get_length()
            }),
        );
    }
}
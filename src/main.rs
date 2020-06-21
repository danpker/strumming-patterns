use clap::Clap;
use rand::prelude::*;

#[derive(Clap)]
#[clap(name = "basic")]
struct Opts {
    /// The number of beats per bar.
    #[clap(short, long, default_value = "4")]
    beats: u8,
    /// Ensure there is a down strum on the first beat.
    #[clap(short, long)]
    easy: bool,
}

fn main() {
    let opts = Opts::parse();

    let mut header = String::new();
    for beat in 1..=opts.beats {
        let beat_header = format!("{} + ", beat);
        header.push_str(&beat_header);
    }
    println!("1 + 2 + 3 + 4 + ");

    let mut strums = String::new();

    for beat in 1..=opts.beats {
        if random() || (beat == 1 && opts.easy) {
            strums.push_str("D ");
        } else {
            strums.push_str("  ");
        }

        if random() {
            strums.push_str("U ");
        } else {
            strums.push_str("  ");
        }
    }

    println!("{}", strums);
}

use clap::Parser;
use std::{io, thread, time};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value_t = 25)]
    study_mins: i32,
    #[arg(long, default_value_t = 0)]
    study_secs: i32,
    #[arg(long, default_value_t = 5)]
    break_mins: i32,
    #[arg(long, default_value_t = 0)]
    break_secs: i32,
    #[arg(long, default_value_t = 30)]
    long_break_mins: i32,
    #[arg(long, default_value_t = 0)]
    long_break_secs: i32,
}

fn main() {
    let args = Args::parse();
    for i in 0..4 {
        if i == 4 {
            timer(30, 0, "Long break");
        } else {
            timer(args.study_mins, args.study_secs, "Study");
            timer(args.break_mins, args.break_secs, "Break");
        }
    }
}

fn timer(minutes: i32, seconds: i32, activity: &str) {
    let mut total_time = (minutes * 60) + seconds;
    while total_time > 0 {
        total_time = total_time - 1;
        print!("\x1B[2J\x1B[1;1H");
        thread::sleep(time::Duration::from_secs(1));
        println!(
            "🍅 {}: {:?} minutes {:?} seconds left",
            activity,
            (total_time / 60),
            total_time - (total_time / 60) * 60
        );
    }
    announcement(activity);
}

fn announcement(message: &str) {
    print!("\x1B[2J\x1B[1;1H");
    println!("🍅 {} period is over (Press enter to continue)", message);
    let mut empty = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut empty).expect("An error occurred");
}

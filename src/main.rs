use std::{io, thread, time};

fn main() {
    for i in 0..4 {
        if i == 4 {
            timer(30, 0, "Long break");
        } else {
            timer(25, 0, "Study");
            timer(5, 0, "Break");
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

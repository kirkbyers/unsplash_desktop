use std::{env, fs::{canonicalize, File}, io::copy, thread::sleep, time::Duration};
use chrono::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut search_term: String = String::new();
    if let Some(first_arg) = args.get(1) {
        // Use the first argument here
        println!("First argument: {}", first_arg);
        search_term = first_arg.to_string();
    } else {
        println!("No arguments provided.");
    }

    let mut last_ran: i64 = 0;

    loop {
        let now = Local::now().timestamp();
        if now - last_ran > 60*60*12 {
            let url = format!("https://source.unsplash.com/random/?{}", search_term);
            let mut response = reqwest::blocking::get(&url).unwrap();

            let mut image_file = File::create(format!("./{}.jpg", now)).unwrap();
            copy(&mut response, &mut image_file).unwrap();

            let full_path = canonicalize(format!("./{}.jpg", now)).unwrap();
            let full_path_str = full_path.to_str().unwrap();
            println!("{full_path_str:?}");

            println!("{full_path_str:?}");
            wallpaper::set_from_path(full_path_str).unwrap();
            last_ran = now.clone();
        }

        sleep(Duration::from_secs(60))
    }
}

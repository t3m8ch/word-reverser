use std::{io, process};

use word_reverser as wr;

fn main() {
    loop {
        let mut text = String::new();
        io::stdin()
            .read_line(&mut text)
            .unwrap_or_else(|_| {
                eprintln!("Failed to read line");
                process::exit(1);
            });

        if text.trim().is_empty() {
            break;
        }

        println!("{}", wr::flip_text_words(text.trim()));
    }
}

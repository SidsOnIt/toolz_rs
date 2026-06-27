use std::io::{self, Write};

pub fn input(prompt: &str) -> String {
    
    print!("{prompt}");
    
    io::stdout()
        .flush()
        .expect("failed to flush std-out");

    let mut response = String::new();
    
    io::stdin()
        .read_line(&mut response)
        .expect("failed to read line");

    response
        .trim_end()
        .to_string()
}

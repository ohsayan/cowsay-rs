use std::env;
use std::io::*;
fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); // HACK(ohsayan) -> Remove the program's file name
    if args.len() != 1 {
        cowsay("I'm not so smart!\nTell me what to do!\nUsage: cowsay \"What you want to say!\"");
    } else {
        cowsay(&args[0]);
    }
}
fn cowsay(val: &str) {
    let cow = "\n        \\   ^__^\n         \\  (oo)\\_______\n            (__)\\       )\\/\\\n                ||----w |\n                ||     ||\n";
    let mut highest = 0;
    for line in val.lines() {
        if line.len() > highest { highest = line.len()};
    }
    highest += 1; // HACK -> This ensures that we have spaces for the < > brackets too
    print!("  ");
    for _i in 0..highest {
        print!("_");
        std::io::stdout().flush().unwrap();
    }
    println!("");
    for line in val.lines() {
    let line = format!("< {} >", line);
    print!("{}\n", line);
    }
    print!("  ");
    for _i in 0..highest {
        print!("-");
        std::io::stdout().flush().unwrap();
    }
    println!("{}", cow);
}

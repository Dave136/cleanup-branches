use crate::{cli::run, colour::{Colour, paint}};

mod cli;
mod colour;
mod git;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", paint(Colour::Red, &format!("Error: {e}")));
        std::process::exit(1);
    }
}

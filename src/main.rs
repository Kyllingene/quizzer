mod error;
mod pack;
mod question;

use error::*;
use pack::*;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to a quiz file (ends in .json)
    #[clap(short, long, value_parser)]
    quiz: String,

    /// How many questions to do (optional, defaults to all)
    #[clap(short, long, value_parser)]
    count: Option<usize>,
}

fn main() -> Result<(), QuestionError> {
    let args = Args::parse();

    let pack = Pack::from_file(args.quiz)?;

    let wrong: usize;
    let right = pack.quiz(args.count, true, true)?;

    if args.count.is_some() {
        wrong = args.count.unwrap() - right;
    } else {
        wrong = pack.len() - right;
    }

    println!("\n\nYou got {} wrong and {} right", wrong, right);

    Ok(())
}

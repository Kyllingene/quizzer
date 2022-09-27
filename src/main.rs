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

    let right = pack.quiz(args.count, true, true)?;

    let wrong = if args.count.is_some() {
        args.count.unwrap() - right
    } else {
        pack.len() - right
    };

    println!("\n\nYou got {} wrong and {} right", wrong, right);

    Ok(())
}

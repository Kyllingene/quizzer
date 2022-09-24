use std::num::ParseIntError;
use std::io::Write;

type QuestionResult = Result<bool, ParseIntError>;

trait Answer {
    fn check(&self, given: String) -> QuestionResult;
}

impl Answer for String {
    fn check(&self, given: String) -> QuestionResult{
        Ok(*self == given)
    }
}
impl Answer for i32 {
    fn check(&self, given: String) -> QuestionResult {
        Ok(*self == given.parse::<i32>()?)
    }
}

trait Question {
    fn get_answer(&self) -> Box<dyn Answer>;

    fn check(&self, given: String) -> QuestionResult {
        self.get_answer().check(given)
    }

    fn ask(&self) -> QuestionResult;
}

pub struct BasicQuestion {
    prompt: String,
    answer: String,
}

impl Question for BasicQuestion {
    fn get_answer(&self) -> Box<dyn Answer> {
        Box::new(self.answer.clone())
    }

    fn ask(&self) -> QuestionResult {
        print!("{}", self.prompt);
        std::io::stdout().flush().unwrap();

        let mut buf = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut buf).unwrap();
        let given = buf.strip_suffix('\n').unwrap_or(&buf).to_string();

        self.get_answer().check(given)
    }
}

fn main() -> Result<(), ParseIntError> {
    let q = BasicQuestion{
        prompt: "What is the answer to life? : ".to_string(),
        answer: "42".to_string(),
    };

    match q.ask()? {
        true  => println!("\n Congratz!"),
        false => println!("\n Failed!"),
    }

    Ok(())
}

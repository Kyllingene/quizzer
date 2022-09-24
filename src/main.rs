use std::num::ParseIntError;

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
}

pub struct BasicQuestion {
    prompt: String,
    answer: String,
}

impl Question for BasicQuestion {
    fn get_answer(&self) -> Box<dyn Answer> {
        Box::new(self.answer.clone())
    }
}

fn main() {
    println!("Hello, world!");
}

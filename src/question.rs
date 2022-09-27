use crate::error::*;
use std::io::Write;

/// A generic question with any kind of answer
pub trait Question {
    fn get_answer(&self) -> Box<dyn Answer>;
    fn get_prompt(&self) -> String;

    fn check(&self, given: String) -> QuestionResult<bool> {
        self.get_answer().check(given)
    }

    fn ask(&self, warn_wrong: bool) -> QuestionResult<bool> {
        print!("{}", self.get_prompt());
        std::io::stdout().flush().unwrap();

        let mut buf = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut buf).unwrap();
        let given = buf.trim_end().to_string();

        let right = self.get_answer().check(given)?;
        if !right && warn_wrong {
            println!(" Wrong! Correct answer: {}", self.get_answer());
        }

        Ok(right)
    }
}

/// A basic, strict-equality question
pub struct BasicQuestion {
    pub prompt: String,
    pub answer: String,
}

impl Question for BasicQuestion {
    fn get_answer(&self) -> Box<dyn Answer> {
        Box::new(self.answer.clone())
    }

    fn get_prompt(&self) -> String {
        self.prompt.clone()
    }
}

/// A basic integer question
pub struct IntQuestion {
    pub prompt: String,
    pub answer: i64,
}

impl Question for IntQuestion {
    fn get_answer(&self) -> Box<dyn Answer> {
        Box::new(self.answer)
    }

    fn get_prompt(&self) -> String {
        self.prompt.clone()
    }
}

/// A basic multiple-choice question
pub struct MultipleChoiceQuestion {
    pub prompt: String,
    pub answer: String,

    pub choices: Vec<String>,
}

impl MultipleChoiceQuestion {
    pub fn new(prompt: String, answer: String, choices: Vec<&str>) -> MultipleChoiceQuestion {
        MultipleChoiceQuestion {
            prompt: prompt.to_string(),
            answer: answer.to_lowercase().to_string(),
            choices: choices
                .iter()
                .map(|s| s.to_lowercase().to_string())
                .collect(),
        }
    }
}

impl Question for MultipleChoiceQuestion {
    fn get_answer(&self) -> Box<dyn Answer> {
        Box::new(self.answer.clone())
    }

    fn get_prompt(&self) -> String {
        self.prompt.clone()
    }

    fn ask(&self, warn_wrong: bool) -> QuestionResult<bool> {
        print!("{}", self.prompt);
        std::io::stdout().flush().unwrap();

        let mut buf = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut buf).unwrap();
        let mut given = buf.trim_end().to_lowercase().to_string();

        while !self.choices.contains(&given) {
            println!(" Invalid answer!");

            buf.clear();

            print!("{}", self.prompt);
            std::io::stdout().flush().unwrap();
            stdin.read_line(&mut buf).unwrap();
            given = buf.trim_end().to_lowercase().to_string();
        }

        let right = self.get_answer().check(given)?;
        if !right && warn_wrong {
            println!(" Wrong! Correct answer: {}", self.get_answer());
        }

        Ok(right)
    }
}

/// A generic answer
/// This crate provides impls for String and i32
trait Answer: std::fmt::Display {
    fn check(&self, given: String) -> QuestionResult<bool>;
}

impl Answer for String {
    fn check(&self, given: String) -> QuestionResult<bool> {
        Ok(self.to_lowercase() == given.to_lowercase())
    }
}

impl Answer for i64 {
    fn check(&self, given: String) -> QuestionResult<bool> {
        Ok(*self
            == given
                .parse::<Self>()
                .map_err(|e| QuestionError::ParseError(Box::new(e)))?)
    }
}

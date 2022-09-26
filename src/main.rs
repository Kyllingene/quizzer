mod error;
mod pack;
mod question;

use error::*;
use pack::*;
use question::*;

fn main() -> Result<(), QuestionError> {
    let q1 = IntQuestion {
        prompt: "What is the answer to life? : ".to_string(),
        answer: 42,
    };

    let q2 = MultipleChoiceQuestion::new("What letter comes after H? : ", "i", vec!["i", "j", "k"]);

    let q3 = BasicQuestion {
        prompt: "What is 'Hello' in Finnish? : ".to_string(),
        answer: "Terve".to_string(),
    };

    let pack = Pack {
        questions: vec![Box::new(q1), Box::new(q2), Box::new(q3)],
    };

    println!("{}", pack.quiz(Some(5), true, true)?);

    Ok(())
}

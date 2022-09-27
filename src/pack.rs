use serde_json::Value;

use crate::error::*;
use crate::question::*;

/// A question pack
pub struct Pack {
    pub questions: Vec<Box<dyn Question>>,
}

fn choice<T>(list: &Vec<T>) -> &T {
    &list[rand::random::<usize>() % list.len()]
}

impl Pack {
    /// Takes a number of questions, asks that many, then returns the number of answers gotten wrong
    /// If given `None`, runs through the whole list of questions
    pub fn quiz(
        &self,
        num: Option<usize>,
        random: bool,
        warn_wrong: bool,
    ) -> QuestionResult<usize> {
        let mut wrong = 0;
        if num.is_none() {
            for q in &self.questions {
                wrong += q.ask(warn_wrong)? as usize;
            }
        } else {
            for i in 0usize..num.unwrap_or(1usize) {
                if random {
                    wrong += choice(&self.questions).ask(warn_wrong)? as usize;
                } else {
                    wrong += self.questions[i % self.questions.len()].ask(warn_wrong)? as usize;
                }
            }
        }

        Ok(wrong)
    }

    /// Get the length of the quiz
    pub fn len(&self) -> usize {
        self.questions.len()
    }

    /// Load a pack from a JSON file
    pub fn from_file(path: String) -> QuestionResult<Self> {
        Pack::from_json(
            std::fs::read_to_string(path.clone())
                .unwrap_or_else(|_| panic!("Unable to open file: {}", path)),
        )
    }

    /// Load a pack from a JSON string
    pub fn from_json(src: String) -> QuestionResult<Self> {
        let mut questions: Vec<Box<dyn Question>> = Vec::new();

        let json: Value =
            serde_json::from_str(&src).map_err(|err| QuestionError::ParseError(Box::new(err)))?;

        if let Value::Array(qs) = &json["questions"] {
            for q in qs {
                let prompt;
                if let Value::String(p) = &q["prompt"] {
                    prompt = p;
                } else {
                    return Err(QuestionError::NoPrompt);
                }

                if let Value::String(t) = &q["type"] {
                    match t.to_lowercase().as_str() {
                        "basic" => {
                            let answer;
                            if let Value::String(a) = &q["answer"] {
                                answer = a.to_lowercase();
                            } else {
                                return Err(QuestionError::BadAnswer);
                            }
                            questions.push(Box::new(BasicQuestion {
                                prompt: prompt.to_string(),
                                answer: answer.to_string(),
                            }));
                        }
                        "number" => {
                            let answer;
                            if let Value::Number(a) = &q["answer"] {
                                answer = a.as_i64();
                                if answer.is_none() {
                                    return Err(QuestionError::BadAnswer);
                                }
                            } else {
                                return Err(QuestionError::BadAnswer);
                            }
                            questions.push(Box::new(IntQuestion {
                                prompt: prompt.to_string(),
                                answer: answer.unwrap(),
                            }));
                        }
                        "choice" => {
                            let answer;
                            if let Value::String(a) = &q["answer"] {
                                answer = a.to_lowercase();
                            } else {
                                return Err(QuestionError::BadAnswer);
                            }
                            let mut options: Vec<String> = Vec::new();
                            if let Value::Array(os) = &q["choices"] {
                                for option in os {
                                    if let Value::String(o) = option {
                                        options.push(o.to_lowercase());
                                    } else {
                                        return Err(QuestionError::BadChoiceType);
                                    }
                                }
                            }

                            questions.push(Box::new(MultipleChoiceQuestion::new(
                                prompt.to_string(),
                                answer,
                                options.iter().map(|s| s.as_str()).collect(),
                            )));
                        }

                        _ => return Err(QuestionError::BadQuestionType),
                    }
                } else {
                    return Err(QuestionError::BadQuestionType);
                }
            }
        } else {
            return Err(QuestionError::EmptyPack);
        }

        Ok(Pack { questions })
    }
}

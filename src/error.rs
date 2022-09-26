use std::error::Error;

pub type QuestionResult<T> = Result<T, QuestionError>;

#[derive(Debug)]
#[allow(unused)]
pub enum QuestionError {
    /// A generic parsing error wrapper
    ParseError(Box<dyn Error>),

    /// A question pack was loaded that contained no questions
    EmptyPack,

    /// A question pack was loaded that contained a question with no type/an invalid type
    BadQuestionType,

    /// A question pack was loaded that contained a question with no prompt
    NoPrompt,

    /// A question pack was loaded that contained a multi-choice question with a non-string choice
    BadChoiceType,

    /// A question pack was loaded that contained a question with no answer/an invalid answer
    BadAnswer,

    /// A generic error wrapper
    OtherError(Box<dyn Error>),
}

use crate::model::{self, Question, QuestionId};
use std::{collections::HashMap, str::FromStr};

#[derive(Clone)]
pub struct Store {
    pub questions: HashMap<QuestionId, Question>,
}
impl Store {
    pub fn new() -> Self {
        Self {
            questions: Self::init(),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("cant read questions")
    }

    fn add_question(&mut self, question: Question) {
        self.questions.insert(question.get_id(), question);
    }
}

use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use warp::reject::Reject;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct QuestionId(String);

#[derive(Debug)]
pub struct InvalidId;
impl Reject for InvalidId {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

impl Question {
    /// Creates a new [`Question`].
    pub fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
    pub fn get_id(&self) -> QuestionId {
        QuestionId(self.id.0.clone())
    }
}
impl FromStr for QuestionId {
    type Err = std::io::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            true => Err(Error::new(ErrorKind::InvalidInput, "no id provided")),
            false => Ok(QuestionId(id.to_string())),
        }
    }
}

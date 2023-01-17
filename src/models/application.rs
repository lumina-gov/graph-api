use juniper::{GraphQLObject, GraphQLUnion};

#[derive(GraphQLObject)]
pub struct Essay {
    question: String,

    answer: String,
}

#[derive(GraphQLObject)]
pub struct MultipleChoice {
    question: String,
    options: Vec<String>,
    answer: String,
}
#[derive(GraphQLUnion)]
pub enum Question {
    Essay(Essay),
    MultipleChoice(MultipleChoice),
}

#[derive(GraphQLObject)]
pub struct ApplicationInput {
    questions: Vec<Question>,
}

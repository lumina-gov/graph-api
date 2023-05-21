use async_graphql::MergedObject;

mod base;
mod question_assessment;
mod user;

#[derive(MergedObject, Default)]
pub struct Query(
    base::BaseQuery,
    user::UserQuery,
    question_assessment::QuestionAssessmentQuery,
);

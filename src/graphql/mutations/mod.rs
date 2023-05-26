use async_graphql::MergedObject;

mod application;
mod base;
mod password_reset;
mod question_assessment;
mod unit_progress;
mod user;

#[derive(MergedObject, Default)]
pub struct Mutation(
    base::BaseMutation,
    user::UserMutation,
    application::ApplicationMutation,
    question_assessment::QuestionAssessmentMutation,
    unit_progress::UnitProgressMutation,
    password_reset::PasswordResetMutation,
);

use async_graphql::MergedObject;

mod auth_apps;
mod base;
mod question_assessment;
mod unit_progress;
mod user;

#[derive(MergedObject, Default)]
pub struct Query(
    base::BaseQuery,
    user::UserQuery,
    question_assessment::QuestionAssessmentQuery,
    unit_progress::UnitProgressQuery,
    auth_apps::AuthAppsQuery,
);

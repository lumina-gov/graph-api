use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use super::unit_progress::UnitProgress;

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct Course {
    course_progress: UnitProgress,
}

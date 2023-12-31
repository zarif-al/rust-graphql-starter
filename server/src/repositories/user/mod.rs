#[cfg(test)]
mod _mock;
pub mod create;
pub mod delete;
pub mod find_many;
pub mod find_one;
pub mod update;

use crate::entities;
use async_graphql::SimpleObject;

#[derive(SimpleObject)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct GraphQLUser {
    pub id: i32,
    pub created_at: i64,
    pub updated_at: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl From<entities::user::Model> for GraphQLUser {
    fn from(value: entities::user::Model) -> Self {
        GraphQLUser {
            created_at: value.created_at.timestamp(),
            updated_at: value.updated_at.timestamp(),
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            id: value.id,
        }
    }
}

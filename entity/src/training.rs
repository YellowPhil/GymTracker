use sea_orm::{prelude::*, sqlx::types::chrono};
use serde::{Deserialize, Serialize};
use super::user::Entity as User;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "trainings")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: i32,
    user_id: i32,
    #[sea_orm(rs_type="Date", db_type="date")]
    date: chrono::NaiveDate
}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
    )]
    User
}
impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
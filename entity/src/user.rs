use sea_orm::{
    ActiveValue::Set,
    prelude::{async_trait::async_trait, *},
};
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, PartialEq, Eq, Deserialize, Debug, Default, Clone, EnumIter, DeriveActiveEnum,
)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
#[serde(tag = "plan")]
pub enum Plan {
    #[sea_orm(num_value = 15)]
    FifteenTrainings = 15,
    TenTrainings = 10,
    #[default]
    FiveTrainings = 5,
}

#[derive(
    Deserialize, Serialize, PartialEq, Eq, Clone, Debug, Default, EnumIter, DeriveActiveEnum,
)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum UserStatus {
    #[default]
    #[sea_orm(num_value = 0)]
    Default,
    #[sea_orm(num_value = 1)]
    Premium,
}
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    #[sea_orm(unique)]
    pub email: String,
    password: String,
    pub status: UserStatus,
    #[sea_orm(nullable)]
    pub plan: Option<Plan>,
    pub visited_trainings: i32,
}
impl Model {
    // TODO: profile if this is long enogh to send it to tokio::spawn_blocking
    pub fn verify_password<'a>(&self, password: &'a str) -> bool {
        bcrypt::verify(password, &self.password).is_ok()
    }
}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::training::Entity")]
    Training,
}

impl Related<super::training::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Training.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert || self.password.is_set() {
            if let Some(password) = self.password.take() {
                let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST)
                    .map_err(|e| DbErr::Custom(format!("Failed to hash password: {}", e)))?;
                self.password = Set(hashed_password);
            }
        }
        Ok(self)
    }
}

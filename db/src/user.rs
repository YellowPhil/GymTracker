use chrono::NaiveDate;
use entity::training::Entity as Training;
use entity::user::{self, Entity as User, Model, Plan};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter,
};
use thiserror::Error;

// TODO: introduce custom error types
#[derive(Debug, Error)]
pub enum MutateError {
    #[error("User has no active plan")]
    NoPlan,
    #[error("Internal db error")]
    InternalError(#[from] DbErr),
}
pub type ViewError = MutateError;

async fn user_by_id(conn: &DatabaseConnection, user_id: i32) -> Result<Model, DbErr> {
    return match User::find_by_id(user_id).one(conn).await? {
        Some(user) => Ok(user),
        None => Err(DbErr::RecordNotFound("User not found".to_string())),
    };
}

pub struct Mutate;
pub struct View;

impl Mutate {
    pub async fn create_user(
        conn: &DatabaseConnection,
        username: String,
        email: String,
        password: String,
        plan: Option<Plan>,
    ) -> Result<i32, MutateError> {
        let user = entity::user::ActiveModel {
            id: NotSet,
            email: Set(email),
            name: Set(username),
            password: Set(password),
            plan: Set(plan),
            status: Set(user::UserStatus::default()),
            visited_trainings: Set(0),
        };
        let updated_user = user.save(conn).await?;
        Ok(updated_user.id.unwrap())
    }
    pub async fn new_plan(
        conn: &DatabaseConnection,
        user_id: i32,
        new_plan: Plan,
    ) -> Result<(), MutateError> {
        let mut user = user_by_id(conn, user_id).await?.into_active_model();
        user.plan = Set(Some(new_plan));
        user.visited_trainings = Set(0);
        user.update(conn).await?;
        Ok(())
    }

    pub async fn add_training(
        conn: &DatabaseConnection,
        user_id: i32,
        date: NaiveDate,
    ) -> Result<(), MutateError> {
        let mut user = user_by_id(conn, user_id).await?.into_active_model();
        let user_plan = match user.plan.clone().unwrap() {
            Some(p) => p,
            None => return Err(MutateError::NoPlan),
        };
        let visited = user.visited_trainings.unwrap();

        let new_training = entity::training::ActiveModel {
            id: NotSet,
            user_id: Set(user_id),
            date: Set(date),
        };
        new_training.insert(conn).await?;

        user.visited_trainings = Set(visited + 1);
        if visited + 1 >= user_plan as i32 {
            user.plan = Set(None);
        }
        user.save(conn).await?;

        Ok(())
    }
}

impl View {
    pub async fn get_trainings(
        conn: &DatabaseConnection,
        user_id: i32,
    ) -> Result<Vec<entity::training::Model>, ViewError> {
        let user = User::find_by_id(user_id).one(conn).await?.unwrap();
        let trainings = user.find_related(Training).all(conn).await?;

        Ok(trainings)
    }
    pub async fn trainings_left(conn: &DatabaseConnection, user_id: i32) -> Result<i32, ViewError> {
        let user = user_by_id(conn, user_id).await?;
        match user.plan {
            Some(plan) => Ok(plan as i32 - user.visited_trainings),
            None => Err(ViewError::NoPlan),
        }
    }
    pub async fn by_id(
        conn: &DatabaseConnection,
        user_id: i32,
    ) -> Result<Option<Model>, ViewError> {
        Ok(User::find_by_id(user_id).one(conn).await?)
    }
    pub async fn by_username(
        conn: &DatabaseConnection,
        username: &str,
    ) -> Result<Option<Model>, ViewError> {
        Ok(User::find()
            .filter(user::Column::Name.eq(username))
            .one(conn)
            .await?)
    }
    pub async fn by_email(
        conn: &DatabaseConnection,
        email: &str,
    ) -> Result<Option<Model>, ViewError> {
        Ok(User::find()
            .filter(user::Column::Email.eq(email))
            .one(conn)
            .await?)
    }
}

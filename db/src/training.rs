use chrono::NaiveDate;
use sea_orm::DatabaseConnection;
use thiserror::Error;

#[derive(Debug,Error)]
pub enum MutateError {
}

pub struct Mutate;
pub struct View;

impl Mutate {
    pub async fn new_training(conn: &DatabaseConnection, user_id: i32, date: NaiveDate) {
    }
}
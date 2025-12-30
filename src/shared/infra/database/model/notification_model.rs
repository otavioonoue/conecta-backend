use chrono::NaiveDateTime;
use sqlx::{prelude::FromRow, types::Uuid};

#[derive(FromRow)]
pub struct NotificationModel {
	pub id: Uuid,
	pub user_id: Uuid,
	pub title: String,
	pub body: String,
	pub read: bool,
	pub created_at: NaiveDateTime
}
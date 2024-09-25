use crate::db::entity::{
    messages::ActiveModel as MessageActiveModel, messages::Model as MessageModel,
    users::Model as UserModel,
};
use chrono::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};

pub async fn create_message(
    conn: DatabaseConnection,
    user: UserModel,
    message: String,
) -> MessageModel {
    let now = Utc::now().naive_utc();

    let message = MessageActiveModel {
        user_id: Set(user.id),
        message: Set(message),
        suggestion_date: Set(now),
        ..Default::default()
    };

    message.insert(&conn).await.unwrap()
}

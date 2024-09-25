use crate::db::entity::{
    messages::ActiveModel as MessageActiveModel, messages::Model as MessageModel,
    prelude::Messages, users::Model as UserModel,
};
use chrono::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};

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

pub async fn get_all_messages(conn: DatabaseConnection) -> Vec<MessageModel> {
    Messages::find().all(&conn).await.unwrap()
}

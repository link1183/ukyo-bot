use crate::db::get_connection;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;

use crate::db::entity::prelude::Users;
use crate::db::entity::users;
use crate::db::entity::users::ActiveModel;
use crate::db::entity::users::Model;

pub async fn get_all_users() -> Vec<Model> {
    Users::find().all(&get_connection().await).await.unwrap()
}

pub async fn get_user_by_discord_id(discord_id: u64) -> Option<Model> {
    Users::find()
        .filter(users::Column::DiscordId.eq(discord_id))
        .one(&get_connection().await)
        .await
        .unwrap()
}

pub async fn create_user(discord_id: u64) {
    let usr = ActiveModel {
        discord_id: Set(discord_id),
        ..Default::default()
    };

    usr.insert(&get_connection().await).await.unwrap();
}

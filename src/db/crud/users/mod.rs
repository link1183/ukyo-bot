use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;

use crate::db::entity::prelude::Users;
use crate::db::entity::users;
use crate::db::entity::users::ActiveModel;
use crate::db::entity::users::Model;

#[allow(unused)]
pub async fn get_all_users(conn: DatabaseConnection) -> Vec<Model> {
    Users::find().all(&conn).await.unwrap()
}

pub async fn get_user_by_discord_id(conn: DatabaseConnection, discord_id: u64) -> Option<Model> {
    Users::find()
        .filter(users::Column::DiscordId.eq(discord_id))
        .one(&conn)
        .await
        .unwrap()
}

pub async fn create_user(conn: DatabaseConnection, discord_id: u64) -> Model {
    let usr = ActiveModel {
        discord_id: Set(discord_id),
        ..Default::default()
    };

    usr.insert(&conn).await.unwrap()
}

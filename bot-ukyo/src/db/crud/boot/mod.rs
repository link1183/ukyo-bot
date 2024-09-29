use crate::{
    db::entity::{
        boot::{self, ActiveModel as BootActiveModel, Model as BootModel},
        prelude::{Boot, Users},
        users::{self, Model as UserModel},
    },
    types::Leaderboard,
};
use chrono::prelude::*;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    QuerySelect,
};

use super::users::get_user_by_discord_id;

pub async fn create_boot(conn: DatabaseConnection, user: UserModel, prob: f64) -> BootModel {
    let now = Utc::now().naive_utc();

    let boot = BootActiveModel {
        user_id: Set(user.id),
        score: Set(prob),
        date: Set(now),
        ..Default::default()
    };

    boot.insert(&conn).await.unwrap()
}

pub async fn get_leaderboard(conn: DatabaseConnection) -> Vec<Leaderboard> {
    Users::find()
        .column(users::Column::Id)
        .column(users::Column::DiscordId)
        .column(boot::Column::Score)
        .inner_join(Boot)
        .into_model::<Leaderboard>()
        .all(&conn)
        .await
        .unwrap()
}

pub async fn get_all_boots_by_discord_id(
    conn: DatabaseConnection,
    discord_id: u64,
) -> Option<Vec<BootModel>> {
    let user = get_user_by_discord_id(conn.clone(), discord_id).await;

    match user.is_none() {
        true => None,
        false => Some(
            Boot::find()
                .filter(boot::Column::UserId.eq(user.unwrap().id))
                .all(&conn)
                .await
                .unwrap(),
        ),
    }
}

use crate::{
    db::{
        entity::{
            boot::{self, ActiveModel as BootActiveModel, Model as BootModel},
            prelude::{Boot, Users},
            users::{self, Model as UserModel},
        },
        get_connection,
    },
    types::Leaderboard,
};
use chrono::prelude::*;
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter,
    QueryOrder, QuerySelect,
};

use super::users::get_user_by_discord_id;

pub async fn create_boot(user: UserModel, prob: f64) {
    let now = Utc::now().naive_utc();

    let boot = BootActiveModel {
        user_id: Set(user.id),
        score: Set(prob),
        date: Set(now),
        ..Default::default()
    };

    boot.insert(&get_connection().await).await.unwrap();
}

pub async fn get_leaderboard() -> Vec<Leaderboard> {
    Users::find()
        .column(users::Column::Id)
        .column(users::Column::DiscordId)
        .expr_as(Expr::col(boot::Column::Score).max(), "highest_score")
        .inner_join(Boot)
        .group_by(users::Column::Id)
        .group_by(users::Column::DiscordId)
        .order_by_desc(Expr::col(boot::Column::Score).max())
        .into_model::<Leaderboard>()
        .all(&get_connection().await)
        .await
        .unwrap()
}

pub async fn get_all_boots_by_discord_id(discord_id: u64) -> Option<Vec<BootModel>> {
    let user = get_user_by_discord_id(discord_id).await;

    match user.is_none() {
        true => None,
        false => Some(
            Boot::find()
                .filter(boot::Column::UserId.eq(user.unwrap().id))
                .all(&get_connection().await)
                .await
                .unwrap(),
        ),
    }
}

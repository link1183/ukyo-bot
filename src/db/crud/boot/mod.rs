use crate::db::{
    entity::{
        boot, boot::ActiveModel as BootActiveModel, boot::Model as BootModel, prelude::Boot,
        users::Model as UserModel,
    },
    get_connection,
};
use chrono::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};

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

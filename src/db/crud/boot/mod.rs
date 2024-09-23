use crate::db::{
    entity::{boot::ActiveModel as BootActiveModel, users::Model as UserModel},
    get_connection,
};
use chrono::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue::Set};

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

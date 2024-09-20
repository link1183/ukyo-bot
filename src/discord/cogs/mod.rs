use crate::types::{Data, Error};

mod misc;

pub fn get_commands() -> Vec<poise::Command<Data, Error>> {
    vec![misc::boot(), misc::test()]
}

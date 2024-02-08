use core::{common::cfg::data::get_db, db};

use axum::{response::IntoResponse, Json};

pub async fn sign_in() -> &'static str {
    "ssssss"
}

pub async fn sign_up() -> impl IntoResponse {
    let clt = get_db().await;
    let user = clt.user().find_unique(db::user::uid::equals(1)).exec().await;

    Json::from(user)
}

// TODO SMS
pub async fn sms_send() {}
pub async fn sms_verify() {}

// TODO EMAIL
pub async fn mail_send() {}
pub async fn mail_verify() {}

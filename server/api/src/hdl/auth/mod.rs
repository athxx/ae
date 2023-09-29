use core::{common::cfg::db::get_db, db};

use axum::{response::IntoResponse, Json};

pub async fn login() -> &'static str {
    "ssssss"
}

pub async fn register() -> impl IntoResponse {
    let clt = get_db().await;
    let user = clt.user().find_unique(db::user::uid::equals(1)).exec().await;

    Json::from(user)
}

pub async fn regiter_sms_send() {}

pub async fn register_mail_send() {}

pub async fn register_sms_verify() {}

pub async fn register_mail_verify() {}

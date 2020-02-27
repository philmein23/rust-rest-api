use crate::api_error::ApiError;
use crate::db;
use crate::schema::email_verification_token;
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct EmailVerificationTokenMessage {
    pub id: Option<String>,
    pub email: String,
}

#[derive(Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "email_verification_token"]
pub struct EmailVerificationToken {
    pub id: Vec<u8>,
    pub email: String,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

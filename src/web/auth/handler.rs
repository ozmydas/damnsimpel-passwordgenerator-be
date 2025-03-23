use axum::Json;
use serde_json::{json, Value};

use crate::{
    models::account::LoginPayload,
    utils::{
        error::{Error, Result},
        palsu,
    },
};

pub async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    if payload.username != "demo" || payload.password != "password" {
        return Err(Error::Loginfail);
    }

    // set cookies
    // todo

    // test

    // return body
    let body = Json(json!({
        "result" : {
            "success" : true,
            "message": "response return success",
            "data": {
                "password" : palsu::generate_password(true, true, false, false, false)
            }
        }
    }));

    Ok(body)
}

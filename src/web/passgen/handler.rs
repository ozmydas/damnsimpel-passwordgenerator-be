use axum::Json;
use serde_json::{json, Value};

use crate::{
    models::passgen::PassgenPayload,
    utils::{
        error::{Error, Result},
        palsu,
    },
};

pub async fn api_passgen(payload: Json<PassgenPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_passgen : {:#?}", "HANDLER", payload);

    // set cookies
    // todo

    // test

    // return body
    let body = Json(json!({
        "result" : {
            "success" : true,
            "message": "response return success",
            "data": {
                "password" : palsu::generate_password(payload.over12, payload.non_english_word, payload.include_special_chars, payload.include_uppercase, payload.include_number)
            }
        }
    }));

    Ok(body)
}

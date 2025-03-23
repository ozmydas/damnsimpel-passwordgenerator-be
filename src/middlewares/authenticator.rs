use argon2::{password_hash, Argon2, PasswordVerifier};
use axum::{
    extract::Request,
    http::HeaderMap,
    middleware::Next,
    response::{IntoResponse, Response},
};

pub async fn verify_request(head: HeaderMap, req: Request, next: Next) -> Response {
    
    println!("🙂‍↔📨 REQ : {:#?} /n", &req);

    let mut res = next.run(req).await;

    let token = head
        .get("token")
        .map_or("jancok", |val| val.to_str().unwrap());

    let date = head
        .get("date")
        .map_or("2000-01-01", |val| val.to_str().unwrap());

    password_hash::PasswordHash::new(token)
        .and_then(|parsedhash| {
            Argon2::default()
                .verify_password(format!("!xxxxxxxx.{date}").as_bytes(), &parsedhash)
                .map(|checked| println!("🙂‍↔️ HEADERS : {:#?} /n/n🔐 PASS : {:?}", token, checked))
        })
        .or_else(|err| {
            println!("🤯 HEADERS : {:#?} /n/n🔐 PASS NOT MATCH !!!", token);
            Err(err)
        });


    res
}

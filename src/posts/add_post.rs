use mongodb::{Client, bson::doc};
use rocket::{http::Status, serde::json::Json, State};
use regex::Regex;

use crate::{DbOptions, auth_guard, guards::auth_guard::AuthTokenGuard, models::{entities::{post::Post, user::{PublicUser, UserModel}}, schemas::{
            error_schema::{ErrorMessage, HFError},
            response_schema::HFResponse,
        }}, utils::HFResult};

#[post("/post/new", data = "<post>")]
pub async fn add_post(
    mut post: Json<Post>,
    opt: &State<DbOptions>,
    user: AuthTokenGuard,
) -> HFResult<String> {
    let _val = auth_guard!(user);

    let exp = Regex::new(r"^[a-zA-Z0-9_-]*$").unwrap();

    if !exp.is_match(post.slug.as_str()) {
        return Err(HFError::CustomError(ErrorMessage {
            status: Some(Status::BadRequest),
            hint: Some("Slug should not contain unsafe characters. Try hyphens instead".to_string()),
            message: "invalid slug".to_string(),
        }));
    }
    let database = Client::with_options(opt.options.clone())?
    .database("hellfire");

    let authors = database.collection::<UserModel>("users");
    let author = authors.find_one(doc! {
        "name": _val
    }, None).await?;

    post.author = Some(PublicUser::from(author.unwrap()));

    let posts = database
        .collection::<Post>("posts");

    if posts.find_one(doc! {"slug": &post.slug}, None).await?.is_some() {
        return Err(HFError::CustomError(ErrorMessage {
            status: Some(Status::BadRequest),
            hint: Some("The given slug already exists".to_string()),
            message: "invalid slug".to_string(),
        }));
    }
    let res = posts.insert_one(&post.into_inner(), None).await?;

    Ok(HFResponse {
        status: None,
        error_hint_message: None,
        response: res.inserted_id.to_string(),
    })
}

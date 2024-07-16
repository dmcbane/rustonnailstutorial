use serde::Deserialize;

use crate::errors::CustomError;
use axum::{
    extract::Extension,
    response::Redirect,
    Form,
};

// 👇 create new SignUp struct
#[derive(Deserialize )]
pub struct SignUp {
    email: String,
}

// 👇 handle form submission
pub async fn process_form(
    Extension(pool): Extension<db::Pool>,
    Form(form): Form<SignUp>,
) -> Result<Redirect, CustomError> {
    let client = pool.get().await?;

    let email = form.email;
    let _ = db::queries::users::create_user()
        .bind(&client, &email.as_str())
        .await?;

    // 303 redirect to users list
    Ok(Redirect::to("/"))
}

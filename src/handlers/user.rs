use bcrypt::{hash, verify, DEFAULT_COST};
use log::error;
use rocket_db_pools::Connection;
use sqlx::{query, query_as};

use crate::errors::ServiceError;
use crate::request::user::{UpdatePassword, UpdateUser};
use crate::response;
use crate::{models, MySQLDb};

pub async fn get_user(
    username: &str,
    upload_url: &str,
    db: &mut Connection<MySQLDb>,
) -> Result<response::user::User, ServiceError> {
    let user = query_as::<_, models::user::User>("SELECT * FROM user WHERE username = ?")
        .bind(username)
        .fetch_one(&mut ***db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ServiceError::Unauthorized,
            _ => ServiceError::InternalServerError,
        })?;

    let avatar = if user.avatar.starts_with("http") || user.avatar.starts_with("https") {
        user.avatar
    } else {
        format!("{}/{}", upload_url, user.avatar)
    };

    Ok(response::user::User {
        username: user.username,
        nickname: user.nickname,
        avatar,
        email: user.email,
    })
}

pub async fn update_user(
    name: &'_ str,
    user: &UpdateUser<'_>,
    db: &mut Connection<MySQLDb>,
) -> Result<(), ServiceError> {
    let record = query_as::<_, models::user::User>(
        "SELECT username, password, email, avatar, nickname FROM user WHERE username = ?",
    )
    .bind(name)
    .fetch_one(&mut ***db)
    .await?;

    let username = match user.username {
        Some(username) => String::from(username),
        None => record.username,
    };

    let email = match user.email {
        Some(email) => String::from(email),
        None => record.email,
    };

    let avatar = match user.avatar {
        Some(avatar) => String::from(avatar),
        None => record.avatar,
    };

    let nickname = match user.nickname {
        Some(nickname) => String::from(nickname),
        None => record.nickname,
    };

    query("UPDATE user SET username = ?, email = ?, avatar = ?, nickname = ? WHERE username = ?")
        .bind(&username)
        .bind(&email)
        .bind(&avatar)
        .bind(&nickname)
        .bind(name)
        .execute(&mut ***db)
        .await?;

    Ok(())
}

pub async fn update_user_password(
    name: &'_ str,
    user: &UpdatePassword<'_>,
    db: &mut Connection<MySQLDb>,
) -> Result<(), ServiceError> {
    let record = query_as::<_, models::user::User>(
        "SELECT username, password, email, avatar, nickname FROM user WHERE username = ?",
    )
    .bind(name)
    .fetch_one(&mut ***db)
    .await?;

    let valid = verify(user.password, &record.password).map_err(|e| {
        error!("{}", e);

        ServiceError::BadRequest(String::from("Invalid password"))
    })?;

    if !valid {
        return Err(ServiceError::Unauthorized);
    }

    let hashed_password = hash(user.new_password, DEFAULT_COST).unwrap();

    query("UPDATE user SET password = ? WHERE username = ?")
        .bind(&hashed_password)
        .bind(name)
        .execute(&mut ***db)
        .await?;

    Ok(())
}

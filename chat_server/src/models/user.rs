use crate::error::AppError;
use crate::models::User;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use sqlx::PgPool;
use std::mem;

#[allow(unused)]
impl User {
    pub async fn find_by_email(email: &str, pool: &PgPool) -> Result<Option<Self>, AppError> {
        let user = sqlx::query_as(
            r#"
            SELECT id, fullname, email, created_at FROM users WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn create(
        fullname: &str,
        email: &str,
        password: &str,
        pool: &PgPool,
    ) -> Result<Self, AppError> {
        let password_hash = hash_password(password)?;

        let ret = sqlx::query_as(
            r#"
            INSERT INTO users (fullname, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, fullname, email, created_at
            "#,
        )
        .bind(fullname)
        .bind(email)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;

        Ok(ret)
    }

    pub async fn verify(
        email: &str,
        password: &str,
        pool: &PgPool,
    ) -> Result<Option<Self>, AppError> {
        let user: Option<User> = sqlx::query_as(
            r#"
            SELECT id, fullname, email, password_hash, created_at FROM users WHERE email = $1
        "#,
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        match user {
            Some(mut user) => {
                let password_hash: Option<String> = mem::take(&mut user.password_hash);
                let is_valid = verify_password(password, &password_hash.unwrap_or_default())?;
                if is_valid {
                    Ok(Some(user))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}

fn hash_password(password: &str) -> Result<String, AppError> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let argon2 = Argon2::default();
    let password_hash = PasswordHash::new(hash)?;
    let is_valid = argon2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok();

    Ok(is_valid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::User;
    use anyhow::Result;
    use sqlx_db_tester::TestPg;
    use std::path::Path;

    #[test]
    fn hash_password_and_verify_should_work() {
        let password = "password";

        let password_hash = hash_password(password).unwrap();
        assert_eq!(password_hash.len(), 97);

        let is_valid = verify_password(password, &password_hash).unwrap();
        assert!(is_valid);
    }

    #[tokio::test]
    async fn create_and_verify_user_should_work() -> Result<()> {
        let tdb = TestPg::new(
            "postgres://postgres:postgres@localhost:5432".to_string(),
            Path::new("../migrations"),
        );
        let pool = tdb.get_pool().await;

        let fullname = "Alice";
        let email = "alice&acme.org";
        let password = "alice_secret";

        let user = User::create(fullname, email, password, &pool).await?;

        assert_eq!(user.fullname, fullname);
        assert_eq!(user.email, email);
        assert_eq!(user.password_hash, None);

        let user = User::verify(email, password, &pool).await?.unwrap();
        assert_eq!(user.fullname, fullname);
        assert_eq!(user.email, email);
        assert_eq!(user.password_hash, None);

        let user = User::find_by_email(email, &pool).await?.unwrap();
        assert_eq!(user.fullname, fullname);
        assert_eq!(user.email, email);
        assert_eq!(user.password_hash, None);

        Ok(())
    }
}

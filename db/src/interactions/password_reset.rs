use crate::DbConnection;
use crate::models::PasswordResetToken;
use crate::schema::password_reset_tokens;
use diesel::prelude::*;

pub struct PasswordResetTokenInteractor;

impl PasswordResetTokenInteractor {
    pub fn create(
        conn: &mut DbConnection,
        email: &str,
    ) -> Result<PasswordResetToken, diesel::result::Error> {
        let token = match conn {
            DbConnection::Sqlite(conn) => {
                let token = PasswordResetToken::new(email, 1); // Token valid for 1 hour

                diesel::insert_into(password_reset_tokens::table)
                    .values(&token)
                    .execute(conn)?;
                token
            }
            DbConnection::Pg(conn) => {
                let token = PasswordResetToken::new(email, 1); // Token valid for 1 hour

                diesel::insert_into(password_reset_tokens::table)
                    .values(&token)
                    .execute(conn)?;
                token
            }
        };

        Ok(token)
    }

    pub fn find_by_token(
        conn: &mut DbConnection,
        token_str: &str,
    ) -> Result<PasswordResetToken, diesel::result::Error> {
        match conn {
            DbConnection::Sqlite(conn) => password_reset_tokens::table
                .filter(password_reset_tokens::token.eq(token_str))
                .first(conn),
            DbConnection::Pg(conn) => password_reset_tokens::table
                .filter(password_reset_tokens::token.eq(token_str))
                .first(conn),
        }
    }

    pub fn delete_by_token(
        conn: &mut DbConnection,
        token_str: &str,
    ) -> Result<usize, diesel::result::Error> {
        match conn {
            DbConnection::Sqlite(conn) => diesel::delete(password_reset_tokens::table)
                .filter(password_reset_tokens::token.eq(token_str))
                .execute(conn),
            DbConnection::Pg(conn) => diesel::delete(password_reset_tokens::table)
                .filter(password_reset_tokens::token.eq(token_str))
                .execute(conn),
        }
    }

    pub fn delete_expired(conn: &mut DbConnection) -> Result<usize, diesel::result::Error> {
        let now = chrono::Utc::now().naive_utc();

        match conn {
            DbConnection::Sqlite(conn) => diesel::delete(password_reset_tokens::table)
                .filter(password_reset_tokens::expires_at.lt(&now))
                .execute(conn),
            DbConnection::Pg(conn) => diesel::delete(password_reset_tokens::table)
                .filter(password_reset_tokens::expires_at.lt(&now))
                .execute(conn),
        }
    }
}

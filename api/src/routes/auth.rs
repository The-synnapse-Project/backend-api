use crate::auth::guard::ApiKey;
use crate::models::Database;
use log::error;
use rocket::response::content::RawJson;
use rocket::serde::json::Json;
use rocket::{State, post};
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[openapi(tag = "Authentication")]
#[post("/api/auth/login", format = "json", data = "<login>")]
pub async fn login(db: &State<Database>, login: Json<Login>, _api_key: ApiKey) -> RawJson<String> {
    let conn = &mut db::establish_connection(&db.db_url);

    // First try to get user by email
    if let Ok(person) = db::interactions::person::PersonInteractor::get_by_email(conn, &login.email)
    {
        if let Some(password_hash) = &person.password_hash {
            if db::crypto::check_hash(&login.password, password_hash) {
                return RawJson("{\"status\":\"ok\"}".into());
            }
            return RawJson("{\"status\":\"error\",\"message\":\"Invalid Password\"}".into());
        } else {
            return RawJson(
                "{\"status\":\"error\",\"message\":\"This account uses social login\"}".into(),
            );
        }
    }
    RawJson("{\"status\":\"error\",\"message\":\"Invalid Email\"}".into())
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Register {
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: Option<String>,
    pub google_id: Option<String>,
}

#[openapi(tag = "Authentication")]
#[post("/api/auth/register", format = "json", data = "<register>")]
pub async fn register(
    db: &State<Database>,
    register: Json<Register>,
    _api_key: ApiKey,
) -> RawJson<String> {
    let conn = &mut db::establish_connection(&db.db_url);

    // If there's a Google ID, check if a user with this Google ID already exists
    if let Some(g_id) = &register.google_id {
        if db::interactions::person::PersonInteractor::get_by_google_id(conn, g_id).is_ok() {
            return RawJson(
                "{\"status\":\"error\",\"message\":\"Google account already registered\"}".into(),
            );
        }
    }

    // Check if user with this email already exists
    if let Ok(existing_user) =
        db::interactions::person::PersonInteractor::get_by_email(conn, &register.email)
    {
        // If a Google ID is provided and the existing user doesn't have one, update the user
        if let Some(_g_id) = &register.google_id {
            if existing_user.google_id.is_none() {
                // Here we could implement a way to update the user's Google ID
                // But for now, just return an informative message
                return RawJson("{\"status\":\"ok\",\"message\":\"User already exists but does not have a Google ID.\"}".into());
            }
        }
        return RawJson("{\"status\":\"error\",\"message\":\"Email already registered\"}".into());
    }

    // Create the new person
    let password_hash = register.password.as_ref().map(|p| db::crypto::to_hash(p));

    let person = db::models::Person::new(
        &register.name,
        &register.surname,
        &register.email,
        db::models::Role::Alumno,
        password_hash.as_deref(),
        register.google_id.as_deref(),
    );

    // Insert the new person
    if let Err(e) = db::interactions::person::PersonInteractor::new(conn, &person) {
        return RawJson(format!(
            "{{\"status\":\"error\",\"message\":\"Failed to create user: {e}\"}}"
        ));
    }

    let permissions = db::models::Permissions::new(&person.id, true, false, true, false, false);

    if let Err(e) = db::interactions::permissions::PermissionsInteractor::new(conn, &permissions) {
        return RawJson(format!(
            "{{\"status\":\"error\",\"message\":\"Failed to create permissions: {e}\"}}"
        ));
    }

    RawJson("{\"status\":\"ok\",\"message\":\"User registered successfully\"}".into())
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ChangePassword {
    pub email: String,
    pub old_password: String,
    pub new_password: String,
}

#[openapi(tag = "Authentication")]
#[post("/api/auth/change-password", format = "json", data = "<change_pw>")]
pub async fn change_password(
    db: &State<Database>,
    change_pw: Json<ChangePassword>,
    _api_key: ApiKey,
) -> RawJson<String> {
    let conn = &mut db::establish_connection(&db.db_url);

    // Check if the user exists
    if let Ok(mut person) =
        db::interactions::person::PersonInteractor::get_by_email(conn, &change_pw.email)
    {
        // Verify old password
        if let Some(password_hash) = &person.password_hash {
            if !db::crypto::check_hash(&change_pw.old_password, password_hash) {
                return RawJson(
                    "{\"status\":\"error\",\"message\":\"Current password is incorrect\"}".into(),
                );
            }
        } else {
            return RawJson(
                "{\"status\":\"error\",\"message\":\"No password set for this account\"}".into(),
            );
        }

        // Update with new password
        person.password_hash = Some(db::crypto::to_hash(&change_pw.new_password));
        if db::interactions::person::PersonInteractor::update(conn, &person.id, &person).is_ok() {
            return RawJson(
                "{\"status\":\"ok\",\"message\":\"Password changed successfully\"}".into(),
            );
        } else {
            return RawJson(
                "{\"status\":\"error\",\"message\":\"Failed to update password\"}".into(),
            );
        }
    }

    RawJson("{\"status\":\"error\",\"message\":\"User not found\"}".into())
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct PasswordResetRequest {
    pub email: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct PasswordResetVerify {
    pub token: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct PasswordReset {
    pub token: String,
    pub new_password: String,
}

#[openapi(tag = "Authentication")]
#[post(
    "/api/auth/forgot-password",
    format = "json",
    data = "<password_reset_req>"
)]
pub async fn forgot_password(
    db: &State<Database>,
    password_reset_req: Json<PasswordResetRequest>,
    _api_key: ApiKey,
) -> RawJson<String> {
    let conn = &mut db::establish_connection(&db.db_url);

    // Check if the user exists
    if let Ok(person) =
        db::interactions::person::PersonInteractor::get_by_email(conn, &password_reset_req.email)
    {
        // Create a new password reset token
        match db::interactions::password_reset::PasswordResetTokenInteractor::create(
            conn,
            &person.email,
        ) {
            Ok(token) => {
                // Clean up expired tokens
                let _ =
                    db::interactions::password_reset::PasswordResetTokenInteractor::delete_expired(
                        conn,
                    );

                // Send an email with the reset token
                match crate::email::send_password_reset_email(&person.email, &token.token).await {
                    Ok(_) => {
                        return RawJson(
                            "{\"status\":\"ok\",\"message\":\"Password reset email sent\"}".into(),
                        );
                    }
                    Err(e) => {
                        error!("Failed to send password reset email: {}", e);
                        return RawJson(
                            "{\"status\":\"error\",\"message\":\"Failed to send email\"}".into(),
                        );
                    }
                }
            }
            Err(e) => {
                error!("Failed to create password reset token: {}", e);
                return RawJson(
                    "{\"status\":\"error\",\"message\":\"Internal server error\"}".into(),
                );
            }
        }
    }

    // Return a success message even if the email doesn't exist to prevent email enumeration
    RawJson("{\"status\":\"ok\",\"message\":\"Password reset email sent\"}".into())
}

#[openapi(tag = "Authentication")]
#[post("/api/auth/verify-reset-token", format = "json", data = "<verify>")]
pub async fn verify_reset_token(
    db: &State<Database>,
    verify: Json<PasswordResetVerify>,
    _api_key: ApiKey,
) -> RawJson<String> {
    let conn = &mut db::establish_connection(&db.db_url);

    // Find the token
    match db::interactions::password_reset::PasswordResetTokenInteractor::find_by_token(
        conn,
        &verify.token,
    ) {
        Ok(token) => {
            if token.is_valid() {
                RawJson("{\"status\":\"ok\",\"valid\":true}".into())
            } else {
                // Delete expired token
                let _ =
                    db::interactions::password_reset::PasswordResetTokenInteractor::delete_by_token(
                        conn,
                        &verify.token,
                    );
                RawJson("{\"status\":\"ok\",\"valid\":false,\"message\":\"Token expired\"}".into())
            }
        }
        Err(_) => {
            RawJson("{\"status\":\"ok\",\"valid\":false,\"message\":\"Invalid token\"}".into())
        }
    }
}

#[openapi(tag = "Authentication")]
#[post("/api/auth/reset-password", format = "json", data = "<reset>")]
pub async fn reset_password(
    db: &State<Database>,
    reset: Json<PasswordReset>,
    _api_key: ApiKey,
) -> RawJson<String> {
    let conn = &mut db::establish_connection(&db.db_url);

    // Find the token and verify it
    match db::interactions::password_reset::PasswordResetTokenInteractor::find_by_token(
        conn,
        &reset.token,
    ) {
        Ok(token) => {
            if !token.is_valid() {
                // Delete expired token
                let _ =
                    db::interactions::password_reset::PasswordResetTokenInteractor::delete_by_token(
                        conn,
                        &reset.token,
                    );
                return RawJson("{\"status\":\"error\",\"message\":\"Token expired\"}".into());
            }

            // Find the user associated with the token
            match db::interactions::person::PersonInteractor::get_by_email(conn, &token.email) {
                Ok(mut person) => {
                    // Update the password
                    person.password_hash = Some(db::crypto::to_hash(&reset.new_password));

                    // Save the updated user
                    match db::interactions::person::PersonInteractor::update(
                        conn, &person.id, &person,
                    ) {
                        Ok(_) => {
                            // Delete the used token
                            let _ = db::interactions::password_reset::PasswordResetTokenInteractor::delete_by_token(
                                conn,
                                &reset.token,
                            );
                            RawJson(
                                "{\"status\":\"ok\",\"message\":\"Password reset successfully\"}"
                                    .into(),
                            )
                        }
                        Err(e) => {
                            error!("Failed to update user: {}", e);
                            RawJson(
                                "{\"status\":\"error\",\"message\":\"Failed to reset password\"}"
                                    .into(),
                            )
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to find user: {}", e);
                    RawJson("{\"status\":\"error\",\"message\":\"User not found\"}".into())
                }
            }
        }
        Err(_) => RawJson("{\"status\":\"error\",\"message\":\"Invalid token\"}".into()),
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct LinkGoogleAccount {
    pub email: String,
    pub google_email: String,
    pub password: String, // To verify the existing account before linking
}

#[openapi(tag = "Authentication")]
#[post("/api/auth/link-google", format = "json", data = "<link_request>")]
pub async fn link_google_account(
    db: &State<Database>,
    link_request: Json<LinkGoogleAccount>,
    _api_key: ApiKey,
) -> RawJson<String> {
    let conn = &mut db::establish_connection(&db.db_url);

    // Check if the user exists
    if let Ok(mut person) =
        db::interactions::person::PersonInteractor::get_by_email(conn, &link_request.email)
    {
        // Verify password
        if let Some(password_hash) = &person.password_hash {
            if !db::crypto::check_hash(&link_request.password, password_hash) {
                return RawJson(
                    "{\"status\":\"error\",\"message\":\"Password is incorrect\"}".into(),
                );
            }
        } else {
            return RawJson(
                "{\"status\":\"error\",\"message\":\"This account has no password set\"}".into(),
            );
        }

        // Check if Google account already exists
        if db::interactions::person::PersonInteractor::get_by_email(
            conn,
            &link_request.google_email,
        )
        .is_ok()
        {
            return RawJson(
                "{\"status\":\"error\",\"message\":\"This Google account is already linked to another user\"}".into(),
            );
        }

        // Update email to the Google email
        // In a production system, you'd want to store both emails and have a proper account linking system
        // This is a simplified approach
        person.email = link_request.google_email.clone();

        if db::interactions::person::PersonInteractor::update(conn, &person.id, &person).is_ok() {
            return RawJson(
                "{\"status\":\"ok\",\"message\":\"Google account linked successfully\"}".into(),
            );
        } else {
            return RawJson("{\"status\":\"error\",\"message\":\"Failed to link account\"}".into());
        }
    }

    RawJson("{\"status\":\"error\",\"message\":\"User not found\"}".into())
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SetPassword {
    pub email: String,
    pub new_password: String,
}

#[openapi(tag = "Authentication")]
#[post("/api/auth/set-password", format = "json", data = "<set_password>")]
pub async fn set_password(
    db: &State<Database>,
    set_password: Json<SetPassword>,
    _api_key: ApiKey,
) -> RawJson<String> {
    let conn = &mut db::establish_connection(&db.db_url);

    // Check if the user exists
    if let Ok(mut person) =
        db::interactions::person::PersonInteractor::get_by_email(conn, &set_password.email)
    {
        // Update with new password
        person.password_hash = Some(db::crypto::to_hash(&set_password.new_password));

        if db::interactions::person::PersonInteractor::update(conn, &person.id, &person).is_ok() {
            return RawJson("{\"status\":\"ok\",\"message\":\"Password set successfully\"}".into());
        } else {
            return RawJson("{\"status\":\"error\",\"message\":\"Failed to set password\"}".into());
        }
    }

    RawJson("{\"status\":\"error\",\"message\":\"User not found\"}".into())
}

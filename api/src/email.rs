use resend_rs::types::CreateEmailBaseOptions;
use resend_rs::{Resend, Result};
use std::env;
use std::error::Error;

pub async fn send_password_reset_email(email: &str, token: &str) -> Result<(), Box<dyn Error>> {
    let api_key = env::var("RESEND_API_KEY").expect("RESEND_API_KEY must be set");
    let client = Resend::new(&api_key);

    let base_url =
        env::var("BASE_URL").unwrap_or_else(|_| "https://syn.loseardes77.dev".to_string());
    let from_email =
        env::var("FROM_EMAIL").unwrap_or_else(|_| "no-reply@syn.loseardes77.dev".to_string());

    // Build the reset URL
    let reset_url = format!("{}/reset-password?token={}", base_url, token);

    // Create email HTML body
    let email_body = format!(
        r#"
        <div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
            <h2 style="color: #333; text-align: center;">Restablecer Contraseña</h2>
            <p style="color: #666; line-height: 1.6;">
                Has solicitado restablecer tu contraseña. Haz clic en el siguiente enlace para crear una nueva contraseña:
            </p>
            <div style="text-align: center; margin: 30px 0;">
                <a href="{reset_url}" 
                style="background-color: #007bff; color: white; padding: 12px 30px; text-decoration: none; border-radius: 5px; display: inline-block;">
                    Restablecer Contraseña
                </a>
            </div>
            <p style="color: #666; line-height: 1.6;">
                Este enlace expirará en 1 hora por motivos de seguridad.
            </p>
            <p style="color: #666; line-height: 1.6;">
                Si no solicitaste restablecer tu contraseña, puedes ignorar este correo electrónico.
            </p>
            <hr style="margin: 30px 0; border: none; border-top: 1px solid #eee;">
            <p style="color: #999; font-size: 12px; text-align: center;">
                Synnapse - Sistema de Gestión Académica
            </p>
        </div>
    "#,
        reset_url = reset_url
    );

    // Create the email request
    let email_request =
        CreateEmailBaseOptions::new(from_email, vec![email], "Restablecer Contraseña - Synnapse")
            .with_html(&email_body);

    // Send the email
    client.emails.send(email_request).await?;

    Ok(())
}

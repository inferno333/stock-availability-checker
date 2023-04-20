use dotenv::dotenv;
use lettre::{smtp::authentication::Credentials, SmtpClient, SmtpTransport, Transport};
use lettre_email::EmailBuilder;
use std::process::Command;

use crate::error::{ErrorKind, Errors};

#[allow(dead_code)]
pub struct Alerter<'a> {
    email: String,
    password: String,
    to_email: &'a str,
    mailer: SmtpTransport,
    mob_no: u64,
}

impl<'a> Alerter<'a> {
    pub fn new(to_email: &'a str, mob_no: u64) -> Result<Alerter<'a>, Errors<'a>> {
        dotenv().ok();
        let email = std::env::var("EMAIL").unwrap();
        let password = std::env::var("PASSWORD").unwrap();
        let creds = Credentials::new(email.clone(), password.clone());
        let mailer = match SmtpClient::new_simple("smtp.gmail.com") {
            Ok(val) => val.credentials(creds).transport(),
            Err(_) => {
                return Err(Errors::new(
                    ErrorKind::Alerterror,
                    "Cannot Connect to SMTP Server",
                ))
            }
        };

        Ok(Alerter {
            email,
            password,
            to_email,
            mailer,
            mob_no,
        })
    }

    pub fn alert_mail(&mut self, link: &'a str) -> Result<(), Errors<'a>> {
        let mail = EmailBuilder::new();
        let
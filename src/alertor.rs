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
    pub fn ne
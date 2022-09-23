use dotenv::dotenv;
use lettre::{smtp::authentication::Credentials, SmtpClient, SmtpTransport, Transport};
use lettre_email::EmailBuilder;
use std::process::Command;

use crate::error::{ErrorKind, Errors};

#[allow(dead_code)]
pub

use reqwest::Client;
use scraper::{Html, Selector};

use crate::alertor::Alerter;
use crate::error::{ErrorKind, Errors};
#[derive(Debug)]
pub struct SiteInfo<'a> {
    name: &'a str,
    address: &'a str,
    css_selector: &'a str,
}

impl<'a> SiteInfo<'a> {
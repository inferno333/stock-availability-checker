
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
    pub fn new(name: &'a str, address: &'a str, css_selector: &'a str) -> SiteInfo<'a> {
        SiteInfo {
            name,
            address,
            css_selector,
        }
    }

    async fn parse_selector(&self) -> Result<Selector, Errors<'a>> {
        Selector::parse(&self.css_selector)
            .map_err(|_| Errors::new(ErrorKind::ParsingError, self.css_selector))
    }

    // pub fn name(&self) -> &'a str {
    //     self.name
    // }
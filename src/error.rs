use std::{
    fmt::{self, Display, Formatter},
    writeln,
};

#[derive(Debug)]
pub enum ErrorKind {
    HtmlError,
    ParsingError,
    Alerterror,
}

pub struct Errors<'a> {
    kind: ErrorKind,
    message: &'a str,
}

impl<'a> Display for 
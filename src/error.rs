use std::{
    fmt::{self, Display, Formatter},
    writeln,
};

#[derive(Debug)]
pub enum ErrorKind {
    HtmlE
pub mod deserialization;
mod normalization;
mod validation;

use crate::dsl::schema::DisplayInformation;
use regex::Regex;
use serde_derive::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
pub enum IntegerBound {
    Inclusive(i64),
    Exclusive(i64),
}

#[derive(Clone, Copy, Debug)]
pub struct IntegerObjectBounds {
    pub minimum: Option<IntegerBound>,
    pub maximum: Option<IntegerBound>,
    pub multiple_of: Option<i64>,
}

impl IntegerBound {
    pub fn inner(&self) -> &i64 {
        match self {
            IntegerBound::Exclusive(value) => value,
            IntegerBound::Inclusive(value) => value,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StringLength {
    pub minimum: Option<i64>,
    pub maximum: Option<i64>,
}

#[derive(Clone, Debug)]
pub enum StringObjectBounds {
    PossibleValues(Vec<EnumerationValue>),
    Pattern(Regex),
    Length(StringLength),
}

#[derive(Clone, Debug)]
pub enum BooleanObjectBounds {
    DefaultValue(bool),
}

#[derive(Clone, Debug)]
pub struct EnumerationValue {
    pub display_information: DisplayInformation,
    pub value: String,
}

impl From<&str> for EnumerationValue {
    fn from(value: &str) -> Self {
        let value = value.to_string();
        let display_information = DisplayInformation {
            title: Some(value.clone()),
            help: None,
            warning: None,
            description: None,
        };
        EnumerationValue {
            value: value.clone(),
            display_information,
        }
    }
}

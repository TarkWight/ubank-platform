use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AppKind {
    Client,
    Employee,
}

impl AppKind {
    pub fn as_db_str(self) -> &'static str {
        match self {
            Self::Client => "client",
            Self::Employee => "employee",
        }
    }
}

impl FromStr for AppKind {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "client" => Ok(Self::Client),
            "employee" => Ok(Self::Employee),
            _ => Err(format!("Unsupported app kind: {value}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Theme {
    pub fn as_db_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
            Self::System => "system",
        }
    }
}

impl FromStr for Theme {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "light" => Ok(Self::Light),
            "dark" => Ok(Self::Dark),
            "system" => Ok(Self::System),
            _ => Err(format!("Unsupported theme: {value}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Client,
    Employee,
}

impl Role {
    pub fn from_claim(value: &str) -> Result<Self, &'static str> {
        match value {
            "CLIENT" => Ok(Self::Client),
            "EMPLOYEE" => Ok(Self::Employee),
            _ => Err("unsupported role"),
        }
    }
}
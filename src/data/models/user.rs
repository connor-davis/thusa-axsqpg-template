use anyhow::Error;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::authentication::roles::Role;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub role: String,
    pub active: bool,
    pub mfa_enabled: bool,
    #[serde(skip_serializing)]
    pub mfa_secret: Option<String>,
    pub mfa_verified: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    #[allow(unused)]
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let val = bincode::serialize(&self).map_err(|error| {
            tracing::error!("🔥 Failed to serialize user: {}", error);
            error
        })?;

        Ok(val)
    }

    #[allow(unused)]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let val = bincode::deserialize(bytes).map_err(|error| {
            tracing::error!("🔥 Failed to deserialize user: {}", error);
            error
        })?;

        Ok(val)
    }

    #[allow(unused)]
    pub fn role(&self) -> Role {
        match self.role.as_str() {
            "Admin" => Role::Admin,
            "System Admin" => Role::SystemAdmin,
            "Customer" => Role::Customer,
            _ => Role::Customer,
        }
    }
}

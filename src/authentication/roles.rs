use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Role {
    Customer,
    Admin,
    SystemAdmin,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Customer => write!(f, "Customer"),
            Role::Admin => write!(f, "Admin"),
            Role::SystemAdmin => write!(f, "System Admin"),
        }
    }
}

use uuid::Uuid;

use crate::domain::settings::enums::Role;

#[derive(Debug, Clone)]
pub struct AuthContext {
    pub user_id: Uuid,
    pub roles: Vec<Role>,
}
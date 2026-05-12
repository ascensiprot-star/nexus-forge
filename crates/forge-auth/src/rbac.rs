use forge_core::error::{ForgeError, ForgeResult};
use forge_core::types::*;

pub struct AuthService {
    users: std::collections::HashMap<UserId, UserRecord>,
}

pub struct UserRecord {
    pub user_id: UserId,
    pub name: String,
    pub role: Role,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            users: std::collections::HashMap::new(),
        }
    }

    pub fn add_user(&mut self, user_id: UserId, name: String, role: Role) {
        self.users.insert(
            user_id.clone(),
            UserRecord {
                user_id,
                name,
                role,
            },
        );
    }

    pub fn check_permission(
        &self,
        user_id: &UserId,
        action: &str,
        resource: &str,
    ) -> ForgeResult<()> {
        let user = self
            .users
            .get(user_id)
            .ok_or_else(|| ForgeError::NotFound {
                entity: "user".to_string(),
                id: user_id.to_string(),
            })?;

        if Self::role_allows(&user.role, action) {
            Ok(())
        } else {
            Err(ForgeError::PermissionDenied {
                action: action.to_string(),
                resource: resource.to_string(),
            })
        }
    }

    fn role_allows(role: &Role, action: &str) -> bool {
        match role {
            Role::Owner | Role::Admin => true,
            Role::Developer => {
                !matches!(action, "delete_project" | "manage_users" | "manage_secrets")
            }
            Role::Reviewer => matches!(action, "view" | "review" | "comment" | "approve"),
            Role::Viewer => action == "view",
        }
    }
}

impl Default for AuthService {
    fn default() -> Self {
        Self::new()
    }
}

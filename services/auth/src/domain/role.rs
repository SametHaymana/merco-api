use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Permission {
    pub resource: String,
    pub action: String,
}

impl Permission {
    pub fn new(resource: impl Into<String>, action: impl Into<String>) -> Self {
        Self {
            resource: resource.into(),
            action: action.into(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}", self.resource, self.action)
    }

    pub fn from_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() == 2 {
            Some(Self {
                resource: parts[0].to_string(),
                action: parts[1].to_string(),
            })
        } else {
            None
        }
    }

    pub fn matches(&self, pattern: &str) -> bool {
        if pattern == "*" || pattern == "*:*" {
            return true;
        }

        if let Some(perm) = Permission::from_string(pattern) {
            if perm.resource == "*" || perm.resource == self.resource {
                if perm.action == "*" || perm.action == self.action {
                    return true;
                }
            }
        }

        false
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: uuid::Uuid,
    pub project_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub permissions: HashSet<Permission>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Role {
    pub fn new(project_id: uuid::Uuid, name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            project_id,
            name,
            description: None,
            permissions: HashSet::new(),
            created_at: chrono::Utc::now(),
        }
    }

    pub fn with_permissions(mut self, permissions: Vec<Permission>) -> Self {
        self.permissions = permissions.into_iter().collect();
        self
    }

    pub fn add_permission(&mut self, permission: Permission) {
        self.permissions.insert(permission);
    }

    pub fn remove_permission(&mut self, permission: &Permission) {
        self.permissions.remove(permission);
    }

    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions.iter().any(|p| p.matches(&permission.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_creation() {
        let perm = Permission::new("posts", "read");
        assert_eq!(perm.resource, "posts");
        assert_eq!(perm.action, "read");
    }

    #[test]
    fn test_permission_string() {
        let perm = Permission::new("posts", "write");
        assert_eq!(perm.to_string(), "posts:write");
    }

    #[test]
    fn test_permission_from_string() {
        let perm = Permission::from_string("posts:delete").unwrap();
        assert_eq!(perm.resource, "posts");
        assert_eq!(perm.action, "delete");
    }

    #[test]
    fn test_permission_wildcard_match() {
        let perm = Permission::new("posts", "read");
        assert!(perm.matches("*"));
        assert!(perm.matches("*:*"));
        assert!(perm.matches("posts:*"));
        assert!(perm.matches("*:read"));
        assert!(perm.matches("posts:read"));
        assert!(!perm.matches("users:read"));
    }

    #[test]
    fn test_role_permissions() {
        let mut role = Role::new(uuid::Uuid::new_v4(), "editor".to_string());
        role.add_permission(Permission::new("posts", "read"));
        role.add_permission(Permission::new("posts", "write"));

        assert!(role.has_permission(&Permission::new("posts", "read")));
        assert!(role.has_permission(&Permission::new("posts", "write")));
        assert!(!role.has_permission(&Permission::new("posts", "delete")));
    }
}

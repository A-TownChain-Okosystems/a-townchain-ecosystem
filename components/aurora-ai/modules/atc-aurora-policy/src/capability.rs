use crate::{Permission, ApprovalRequirement};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapabilityId(String);

impl CapabilityId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for CapabilityId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capability {
    pub id: CapabilityId,
    pub operation: String,
    pub resource: String,
    pub permission: Permission,
    pub approval: ApprovalRequirement,
    pub audit_required: bool,
}

impl Capability {
    pub fn new(
        id: impl Into<String>,
        operation: impl Into<String>,
        resource: impl Into<String>,
        permission: impl Into<Permission>,
        approval: ApprovalRequirement,
        audit_required: bool,
    ) -> Self {
        Self {
            id: CapabilityId::new(id),
            operation: operation.into(),
            resource: resource.into(),
            permission: permission.into(),
            approval,
            audit_required,
        }
    }

    pub fn permits(&self, request: &CapabilityRequest) -> bool {
        self.id == request.capability
            && self.operation == request.operation
            && self.permission == request.permission
            && resource_matches(&self.resource, &request.resource)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityRequest {
    pub capability: CapabilityId,
    pub operation: String,
    pub resource: String,
    pub permission: Permission,
}

impl CapabilityRequest {
    pub fn new(
        capability: impl Into<CapabilityId>,
        operation: impl Into<String>,
        resource: impl Into<String>,
    ) -> Self {
        Self {
            capability: capability.into(),
            operation: operation.into(),
            resource: resource.into(),
            permission: Permission::new(""),
        }
    }

    pub fn with_permission(mut self, permission: impl Into<Permission>) -> Self {
        self.permission = permission.into();
        self
    }
}

fn resource_matches(pattern: &str, resource: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if let Some(prefix) = pattern.strip_suffix('*') {
        return resource.starts_with(prefix);
    }
    pattern == resource
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capability_matches_scoped_resource() {
        let capability = Capability::new(
            "filesystem.read",
            "read",
            "/workspace/project/*",
            "filesystem.read",
            ApprovalRequirement::None,
            true,
        );
        assert!(capability.permits(&CapabilityRequest::new(
            "filesystem.read",
            "read",
            "/workspace/project/src/lib.rs"
        ).with_permission("filesystem.read")));
        assert!(!capability.permits(&CapabilityRequest::new(
            "filesystem.read",
            "read",
            "/etc/passwd"
        ).with_permission("filesystem.read")));
    }
}

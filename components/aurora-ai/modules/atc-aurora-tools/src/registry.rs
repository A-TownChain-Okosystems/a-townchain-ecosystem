use std::collections::HashMap;

use crate::{Tool, ToolError};

#[derive(Default)]
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<T>(&mut self, tool: T)
    where
        T: Tool + 'static,
    {
        self.tools.insert(tool.id().to_owned(), Box::new(tool));
    }

    pub fn get(&self, id: &str) -> Option<&dyn Tool> {
        self.tools.get(id).map(|tool| tool.as_ref())
    }

    pub fn require(&self, id: &str) -> Result<&dyn Tool, ToolError> {
        self.get(id)
            .ok_or_else(|| ToolError::ExecutionFailed(format!("tool '{}' not registered", id)))
    }

    pub fn len(&self) -> usize {
        self.tools.len()
    }
}

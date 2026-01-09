use crate::types::*;
use chrono::Utc;
use dashmap::DashMap;
use parking_lot::RwLock;
use std::sync::Arc;
use std::path::{Path, PathBuf};
use anyhow::Result;
use serde_json;

pub struct MemoryManager {
    task_memory: Arc<DashMap<String, Task>>,
    project_memory: Arc<DashMap<String, ProjectMemory>>,
    system_memory: Arc<RwLock<SystemMemory>>,
    storage_path: PathBuf,
}

impl MemoryManager {
    pub fn new(storage_path: impl AsRef<Path>) -> Result<Self> {
        let path = storage_path.as_ref().to_path_buf();
        std::fs::create_dir_all(&path)?;

        let system_memory = SystemMemory {
            app_schemas: std::collections::HashMap::new(),
            safety_rules: Vec::new(),
            workflow_templates: Vec::new(),
            version: "1.0.0".to_string(),
            last_updated: Utc::now(),
        };

        Ok(Self {
            task_memory: Arc::new(DashMap::new()),
            project_memory: Arc::new(DashMap::new()),
            system_memory: Arc::new(RwLock::new(system_memory)),
            storage_path: path,
        })
    }

    pub fn store_task_memory(&self, task: &Task) -> Result<()> {
        // Store in-memory
        self.task_memory.insert(task.task_id.clone(), task.clone());

        // Persist to disk
        let task_path = self.storage_path.join("tasks").join(format!("{}.json", task.task_id));
        std::fs::create_dir_all(task_path.parent().unwrap())?;
        let json = serde_json::to_string_pretty(task)?;
        std::fs::write(&task_path, json)?;

        Ok(())
    }

    pub fn get_task_memory(&self, task_id: &str) -> Option<Task> {
        // Try in-memory first
        if let Some(task) = self.task_memory.get(task_id) {
            return Some(task.clone());
        }

        // Try disk
        let task_path = self.storage_path.join("tasks").join(format!("{}.json", task_id));
        if task_path.exists() {
            if let Ok(json) = std::fs::read_to_string(&task_path) {
                if let Ok(task) = serde_json::from_str::<Task>(&json) {
                    self.task_memory.insert(task_id.to_string(), task.clone());
                    return Some(task);
                }
            }
        }

        None
    }

    pub fn store_project_memory(&self, project: &ProjectMemory) -> Result<()> {
        self.project_memory.insert(project.project_id.clone(), project.clone());

        let project_path = self.storage_path.join("projects").join(format!("{}.json", project.project_id));
        std::fs::create_dir_all(project_path.parent().unwrap())?;
        let json = serde_json::to_string_pretty(project)?;
        std::fs::write(&project_path, json)?;

        Ok(())
    }

    pub fn get_project_memory(&self, project_id: &str) -> Option<ProjectMemory> {
        if let Some(project) = self.project_memory.get(project_id) {
            return Some(project.clone());
        }

        let project_path = self.storage_path.join("projects").join(format!("{}.json", project_id));
        if project_path.exists() {
            if let Ok(json) = std::fs::read_to_string(&project_path) {
                if let Ok(project) = serde_json::from_str::<ProjectMemory>(&json) {
                    self.project_memory.insert(project_id.to_string(), project.clone());
                    return Some(project);
                }
            }
        }

        None
    }

    pub fn get_system_memory(&self) -> SystemMemory {
        self.system_memory.read().clone()
    }

    pub fn update_system_memory<F>(&self, f: F) -> Result<()>
    where
        F: FnOnce(&mut SystemMemory),
    {
        let mut memory = self.system_memory.write();
        f(&mut memory);
        memory.last_updated = Utc::now();

        // Persist
        let system_path = self.storage_path.join("system_memory.json");
        let json = serde_json::to_string_pretty(&*memory)?;
        std::fs::write(&system_path, json)?;

        Ok(())
    }

    pub fn record_workflow_history(
        &self,
        project_id: &str,
        task_id: &str,
        success: bool,
        duration_ms: u64,
    ) -> Result<()> {
        let project = self.get_project_memory(project_id);
        if let Some(mut proj) = project {
            proj.workflow_history.push(WorkflowHistoryEntry {
                task_id: task_id.to_string(),
                executed_at: Utc::now(),
                success,
                duration_ms,
            });
            proj.updated_at = Utc::now();
            self.store_project_memory(&proj)?;
        } else {
            // Create default project if it doesn't exist
            let proj = ProjectMemory {
                project_id: project_id.to_string(),
                project_name: "Default Project".to_string(),
                recurring_rules: Vec::new(),
                workflow_history: vec![WorkflowHistoryEntry {
                    task_id: task_id.to_string(),
                    executed_at: Utc::now(),
                    success,
                    duration_ms,
                }],
                automation_preferences: AutomationPreferences::default(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            self.store_project_memory(&proj)?;
        }

        Ok(())
    }

    pub fn get_app_schema(&self, domain: &str) -> Option<AppSchema> {
        let memory = self.system_memory.read();
        memory.app_schemas.get(domain).cloned()
    }

    pub fn update_app_schema(&self, domain: &str, schema: AppSchema) -> Result<()> {
        self.update_system_memory(|memory| {
            memory.app_schemas.insert(domain.to_string(), schema);
        })
    }
}


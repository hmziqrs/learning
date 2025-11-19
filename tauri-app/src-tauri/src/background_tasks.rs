use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundTask {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub task_type: TaskType,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub created_at: u64,
    pub scheduled_for: Option<u64>,
    pub last_run: Option<u64>,
    pub next_run: Option<u64>,
    pub result: Option<TaskResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TaskType {
    OneTime,
    Periodic,
    Triggered,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskPriority {
    High,
    Normal,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
    pub executed_at: u64,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskSchedule {
    #[serde(rename = "type")]
    pub task_type: TaskType,
    pub start_time: Option<u64>,
    pub interval_ms: Option<u64>,
    pub end_time: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConstraints {
    pub requires_network: Option<bool>,
    pub requires_wifi: Option<bool>,
    pub requires_charging: Option<bool>,
    pub requires_battery_not_low: Option<bool>,
    pub requires_device_idle: Option<bool>,
    pub requires_storage_not_low: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskOptions {
    pub name: String,
    pub description: Option<String>,
    pub schedule: TaskSchedule,
    pub priority: Option<TaskPriority>,
    pub constraints: Option<TaskConstraints>,
    pub retry_policy: Option<RetryPolicy>,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub backoff_multiplier: Option<f64>,
    pub initial_backoff_ms: Option<u64>,
    pub max_backoff_ms: Option<u64>,
}

pub struct TaskManager {
    tasks: Arc<Mutex<HashMap<String, BackgroundTask>>>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_task(&self, options: CreateTaskOptions) -> Result<String, String> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = get_current_timestamp();

        let task = BackgroundTask {
            id: id.clone(),
            name: options.name,
            description: options.description,
            task_type: options.schedule.task_type.clone(),
            status: TaskStatus::Pending,
            priority: options.priority.unwrap_or(TaskPriority::Normal),
            created_at: now,
            scheduled_for: options.schedule.start_time,
            last_run: None,
            next_run: options.schedule.start_time,
            result: None,
        };

        self.tasks
            .lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?
            .insert(id.clone(), task);

        Ok(id)
    }

    pub fn get_task(&self, id: &str) -> Result<Option<BackgroundTask>, String> {
        let tasks = self
            .tasks
            .lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;
        Ok(tasks.get(id).cloned())
    }

    pub fn list_tasks(&self) -> Result<Vec<BackgroundTask>, String> {
        let tasks = self
            .tasks
            .lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;
        Ok(tasks.values().cloned().collect())
    }

    pub fn cancel_task(&self, id: &str) -> Result<(), String> {
        let mut tasks = self
            .tasks
            .lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;

        if let Some(task) = tasks.get_mut(id) {
            if task.status == TaskStatus::Running {
                return Err("Cannot cancel running task".to_string());
            }
            if task.status == TaskStatus::Completed || task.status == TaskStatus::Failed {
                return Err(format!("Cannot cancel {} task", format!("{:?}", task.status).to_lowercase()));
            }
            task.status = TaskStatus::Cancelled;
            Ok(())
        } else {
            Err(format!("Task {} not found", id))
        }
    }

    pub fn delete_task(&self, id: &str) -> Result<(), String> {
        let mut tasks = self
            .tasks
            .lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;

        if let Some(task) = tasks.get(id) {
            if task.status == TaskStatus::Running {
                return Err("Cannot delete running task".to_string());
            }
        }

        tasks.remove(id);
        Ok(())
    }

    pub fn update_task_status(&self, id: &str, status: TaskStatus) -> Result<(), String> {
        let mut tasks = self
            .tasks
            .lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;

        if let Some(task) = tasks.get_mut(id) {
            task.status = status;
            Ok(())
        } else {
            Err(format!("Task {} not found", id))
        }
    }

    pub fn update_task_result(
        &self,
        id: &str,
        status: TaskStatus,
        result: TaskResult,
    ) -> Result<(), String> {
        let mut tasks = self
            .tasks
            .lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;

        if let Some(task) = tasks.get_mut(id) {
            task.status = status;
            task.result = Some(result);
            task.last_run = Some(get_current_timestamp());
            Ok(())
        } else {
            Err(format!("Task {} not found", id))
        }
    }

    pub fn clone_tasks(&self) -> Arc<Mutex<HashMap<String, BackgroundTask>>> {
        Arc::clone(&self.tasks)
    }
}

fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

// Execute a demo task with simulated work
pub async fn execute_demo_task(
    task_manager: Arc<Mutex<HashMap<String, BackgroundTask>>>,
    task_id: String,
    delay_seconds: u64,
) -> Result<(), String> {
    // Update status to Running
    {
        let mut tasks = task_manager
            .lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;
        if let Some(task) = tasks.get_mut(&task_id) {
            task.status = TaskStatus::Running;
            task.last_run = Some(get_current_timestamp());
        } else {
            return Err(format!("Task {} not found", task_id));
        }
    }

    let start_time = get_current_timestamp();

    // Simulate work
    sleep(Duration::from_secs(delay_seconds)).await;

    let duration_ms = get_current_timestamp() - start_time;

    // Randomly succeed or fail for demo purposes (80% success rate)
    let success = (start_time % 10) != 0;

    let result = if success {
        TaskResult {
            success: true,
            data: Some(serde_json::json!({
                "message": "Task completed successfully",
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "delay_seconds": delay_seconds
            })),
            error: None,
            executed_at: get_current_timestamp(),
            duration_ms,
        }
    } else {
        TaskResult {
            success: false,
            data: None,
            error: Some("Simulated task failure".to_string()),
            executed_at: get_current_timestamp(),
            duration_ms,
        }
    };

    // Update task with result
    {
        let mut tasks = task_manager
            .lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;
        if let Some(task) = tasks.get_mut(&task_id) {
            task.status = if success {
                TaskStatus::Completed
            } else {
                TaskStatus::Failed
            };
            task.result = Some(result);
        }
    }

    Ok(())
}

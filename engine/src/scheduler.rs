use crate::types::*;
use crate::task_manager::TaskManager;
use chrono::{DateTime, Utc, Duration as ChronoDuration, Datelike};
use std::sync::Arc;
use tokio::time::{interval, Duration};
use anyhow::Result;
use dashmap::DashMap;

pub struct Scheduler {
    task_manager: Arc<TaskManager>,
    scheduled_tasks: Arc<DashMap<String, ScheduledTaskInfo>>,
}

struct ScheduledTaskInfo {
    task_id: String,
    next_run: DateTime<Utc>,
    recurrence: Option<Recurrence>,
}

impl Scheduler {
    pub fn new(task_manager: Arc<TaskManager>) -> Self {
        Self {
            task_manager,
            scheduled_tasks: Arc::new(DashMap::new()),
        }
    }

    pub fn register_scheduled_task(&self, task_id: String, scheduling: Scheduling) -> Result<()> {
        if !scheduling.enabled {
            return Ok(());
        }

        let next_run = scheduling.next_run;
        let recurrence = scheduling.recurrence;

        self.scheduled_tasks.insert(task_id.clone(), ScheduledTaskInfo {
            task_id: task_id.clone(),
            next_run,
            recurrence,
        });

        Ok(())
    }

    pub fn unregister_scheduled_task(&self, task_id: &str) {
        self.scheduled_tasks.remove(task_id);
    }

    pub async fn start_scheduler_loop(&self) -> Result<()> {
        let mut interval = interval(Duration::from_secs(60)); // Check every minute

        loop {
            interval.tick().await;
            self.check_and_trigger_tasks().await?;
        }
    }

    async fn check_and_trigger_tasks(&self) -> Result<()> {
        let now = Utc::now();
        let mut tasks_to_trigger = Vec::new();

        for entry in self.scheduled_tasks.iter() {
            let info = entry.value();
            if info.next_run <= now {
                tasks_to_trigger.push((info.task_id.clone(), info.next_run, info.recurrence.clone()));
            }
        }

        for (task_id, _triggered_at, recurrence) in tasks_to_trigger {
            // Trigger the task
            if let Some(task) = self.task_manager.get_task(&task_id) {
                // Check if task can auto-run (repetitive tasks)
                if task.automation.auto_run_enabled {
                    if let Err(e) = self.task_manager.start_task(&task_id) {
                        eprintln!("Failed to start scheduled task {}: {}", task_id, e);
                    }
                } else {
                    // Task needs approval - mark as pending
                    // In a real implementation, this would notify the UI
                    eprintln!("Scheduled task {} requires approval", task_id);
                }

                // Calculate next run if recurring
                if let Some(recur) = recurrence {
                    if let Some(next_run) = self.calculate_next_run(now, &recur) {
                        if let Some(mut entry) = self.scheduled_tasks.get_mut(&task_id) {
                            entry.next_run = next_run;
                        }
                    } else {
                        // No more runs scheduled
                        self.scheduled_tasks.remove(&task_id);
                    }
                } else {
                    // One-time task, remove from scheduler
                    self.scheduled_tasks.remove(&task_id);
                }
            }
        }

        Ok(())
    }

    fn calculate_next_run(&self, current: DateTime<Utc>, recurrence: &Recurrence) -> Option<DateTime<Utc>> {
        match recurrence.frequency {
            Frequency::Daily => {
                let mut next = current + ChronoDuration::days(1);
                if let Some(time_str) = &recurrence.time {
                    // Parse time and set it
                    if let Some((hour, minute)) = parse_time(time_str) {
                        next = next.date_naive().and_hms_opt(hour, minute, 0)?
                            .and_utc();
                    }
                }
                Some(next)
            }
            Frequency::Weekly => {
                let mut next = current + ChronoDuration::days(7);
                if let Some(days) = &recurrence.days_of_week {
                    // Find next matching day of week
                    let current_weekday = current.weekday().num_days_from_monday() as u8;
                    if let Some(&next_day) = days.iter().find(|&&d| d > current_weekday) {
                        let days_to_add = (next_day - current_weekday) as i64;
                        next = current + ChronoDuration::days(days_to_add);
                    } else if let Some(&first_day) = days.first() {
                        // Next week
                        let days_to_add = (7 - current_weekday + first_day) as i64;
                        next = current + ChronoDuration::days(days_to_add);
                    }
                }
                if let Some(time_str) = &recurrence.time {
                    if let Some((hour, minute)) = parse_time(time_str) {
                        next = next.date_naive().and_hms_opt(hour, minute, 0)?
                            .and_utc();
                    }
                }
                Some(next)
            }
            Frequency::Monthly => {
                Some(current + ChronoDuration::days(30))
            }
            Frequency::Custom => {
                if let Some(interval) = recurrence.interval {
                    Some(current + ChronoDuration::days(interval as i64))
                } else {
                    None
                }
            }
        }
    }

    pub fn get_scheduled_tasks(&self) -> Vec<(String, DateTime<Utc>)> {
        self.scheduled_tasks.iter()
            .map(|entry| (entry.task_id.clone(), entry.next_run))
            .collect()
    }
}

fn parse_time(time_str: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() == 2 {
        if let (Ok(hour), Ok(minute)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
            if hour < 24 && minute < 60 {
                return Some((hour, minute));
            }
        }
    }
    None
}


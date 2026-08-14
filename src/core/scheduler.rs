use crate::error::Error;
use crate::Result;
/// Scheduler - Priority classes, bounded queues, backpressure
/// Implements: AC §55, SD-12
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PriorityClass {
    Realtime,
    High,
    Normal,
    Low,
    Background,
}

impl PriorityClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            PriorityClass::Realtime => "realtime",
            PriorityClass::High => "high",
            PriorityClass::Normal => "normal",
            PriorityClass::Low => "low",
            PriorityClass::Background => "background",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackpressurePolicy {
    DropOldest,
    DropNewest,
    Sample,
    Merge,
    Compress,
    Block,
    Reject,
}

#[derive(Debug, Clone)]
pub struct QueueConfig {
    pub capacity: usize,
    pub policy: BackpressurePolicy,
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            capacity: 64,
            policy: BackpressurePolicy::DropOldest,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub priority: PriorityClass,
    pub payload: String,
}

pub struct BoundedQueue {
    queue: VecDeque<Task>,
    config: QueueConfig,
    dropped_count: u64,
}

impl BoundedQueue {
    pub fn new(config: QueueConfig) -> Self {
        Self {
            queue: VecDeque::with_capacity(config.capacity),
            config,
            dropped_count: 0,
        }
    }

    pub fn push(&mut self, task: Task) -> Result<()> {
        if self.queue.len() >= self.config.capacity {
            match self.config.policy {
                BackpressurePolicy::DropOldest => {
                    self.queue.pop_front();
                    self.queue.push_back(task);
                }
                BackpressurePolicy::DropNewest => {
                    self.dropped_count += 1;
                    return Ok(());
                }
                BackpressurePolicy::Reject => {
                    self.dropped_count += 1;
                    return Err(Error::InternalOther("Queue full, rejected".to_string()));
                }
                BackpressurePolicy::Block => {
                    return Err(Error::InternalOther("Queue full, block".to_string()));
                }
                BackpressurePolicy::Sample => {
                    if self.queue.len() > 1 {
                        self.queue.pop_front();
                    }
                    self.queue.push_back(task);
                }
                BackpressurePolicy::Merge => {
                    if let Some(last) = self.queue.back_mut() {
                        last.payload.push_str(&format!(";{}", task.payload));
                    } else {
                        self.queue.push_back(task);
                    }
                }
                BackpressurePolicy::Compress => {
                    self.queue.pop_front();
                    self.queue.push_back(task);
                }
            }
        } else {
            self.queue.push_back(task);
        }
        Ok(())
    }

    pub fn pop(&mut self) -> Option<Task> {
        self.queue.pop_front()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.config.capacity
    }

    pub fn dropped_count(&self) -> u64 {
        self.dropped_count
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }
}

pub struct Scheduler {
    pub queues: std::collections::HashMap<PriorityClass, BoundedQueue>,
    pub maintenance_budget_percent: u8,
}

impl Scheduler {
    pub fn new(maintenance_budget_percent: u8) -> Self {
        let mut queues = std::collections::HashMap::new();
        queues.insert(
            PriorityClass::Realtime,
            BoundedQueue::new(QueueConfig {
                capacity: 32,
                policy: BackpressurePolicy::Reject,
            }),
        );
        queues.insert(
            PriorityClass::High,
            BoundedQueue::new(QueueConfig {
                capacity: 64,
                policy: BackpressurePolicy::DropOldest,
            }),
        );
        queues.insert(
            PriorityClass::Normal,
            BoundedQueue::new(QueueConfig {
                capacity: 128,
                policy: BackpressurePolicy::DropOldest,
            }),
        );
        queues.insert(
            PriorityClass::Low,
            BoundedQueue::new(QueueConfig {
                capacity: 256,
                policy: BackpressurePolicy::DropNewest,
            }),
        );
        queues.insert(
            PriorityClass::Background,
            BoundedQueue::new(QueueConfig {
                capacity: 512,
                policy: BackpressurePolicy::DropNewest,
            }),
        );

        Self {
            queues,
            maintenance_budget_percent: maintenance_budget_percent.min(100),
        }
    }

    pub fn submit(&mut self, task: Task) -> Result<()> {
        let queue = self
            .queues
            .get_mut(&task.priority)
            .ok_or_else(|| Error::InternalOther("Unknown priority class".to_string()))?;
        queue.push(task)
    }

    pub fn dequeue(&mut self) -> Option<Task> {
        for priority in [
            PriorityClass::Realtime,
            PriorityClass::High,
            PriorityClass::Normal,
            PriorityClass::Low,
            PriorityClass::Background,
        ] {
            if let Some(queue) = self.queues.get_mut(&priority) {
                if let Some(task) = queue.pop() {
                    return Some(task);
                }
            }
        }
        None
    }

    pub fn queue_len(&self, priority: PriorityClass) -> usize {
        self.queues.get(&priority).map(|q| q.len()).unwrap_or(0)
    }

    pub fn total_queued(&self) -> usize {
        self.queues.values().map(|q| q.len()).sum()
    }

    pub fn can_run_maintenance(&self) -> bool {
        self.maintenance_budget_percent < 100
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new(10)
    }
}

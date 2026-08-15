#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorFormat {
    Raw,
    Float32,
    Float64,
    Int8,
    Int16,
    Int32,
}

#[derive(Debug, Clone)]
pub struct SensorFrame {
    pub sensor_id: u32,
    pub timestamp: u64,
    pub sequence: u64,
    pub payload: Vec<u8>,
    pub dimensions: [u32; 3],
    pub format: SensorFormat,
    pub quality: f32,
    pub flags: u32,
}

impl SensorFrame {
    pub fn new(sensor_id: u32, timestamp: u64, sequence: u64, payload: Vec<u8>) -> Self {
        Self {
            sensor_id,
            timestamp,
            sequence,
            payload,
            dimensions: [0; 3],
            format: SensorFormat::Raw,
            quality: 1.0,
            flags: 0,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.quality > 0.0
            && self.dimensions[0] > 0
            && self.dimensions[1] > 0
            && self.dimensions[2] > 0
    }

    pub fn size_bytes(&self) -> usize {
        self.payload.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropPolicy {
    DropOldest,
    DropNewest,
}

pub struct BoundedBuffer<T> {
    max_items: usize,
    drop_policy: DropPolicy,
    items: Vec<T>,
    dropped_count: u64,
}

impl<T> BoundedBuffer<T> {
    pub fn new(max_items: usize, drop_policy: DropPolicy) -> Self {
        Self {
            max_items,
            drop_policy,
            items: Vec::with_capacity(max_items),
            dropped_count: 0,
        }
    }

    pub fn push(&mut self, item: T) -> bool {
        if self.items.len() < self.max_items {
            self.items.push(item);
            true
        } else {
            match self.drop_policy {
                DropPolicy::DropOldest => {
                    self.items.remove(0);
                    self.items.push(item);
                }
                DropPolicy::DropNewest => {}
            }
            self.dropped_count += 1;
            false
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.items.len() >= self.max_items
    }

    pub fn dropped_count(&self) -> u64 {
        self.dropped_count
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

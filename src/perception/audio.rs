use super::sensor::{BoundedBuffer, DropPolicy, SensorFrame};
use crate::Result;

pub struct AudioConfig {
    pub max_chunks: u32,
    pub max_chunk_bytes: u32,
    pub drop_policy: DropPolicy,
}

pub struct MockAudio {
    config: AudioConfig,
    buffer: BoundedBuffer<SensorFrame>,
    next_sequence: u64,
    running: bool,
}

impl MockAudio {
    pub fn new(config: AudioConfig) -> Self {
        let buffer = BoundedBuffer::new(config.max_chunks as usize, config.drop_policy);
        Self {
            config,
            buffer,
            next_sequence: 0,
            running: false,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        self.running = true;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        self.running = false;
        self.buffer.clear();
        Ok(())
    }

    pub fn capture_chunk(&mut self, samples: u32) -> Result<SensorFrame> {
        if !self.running {
            return Err(crate::Error::PerceptionAudioError(
                "audio not running".into(),
            ));
        }
        let bytes_needed = (samples * 2) as usize;
        let max_bytes = self.config.max_chunk_bytes as usize;
        let effective_bytes = if bytes_needed > max_bytes {
            max_bytes
        } else {
            bytes_needed
        };
        let payload: Vec<u8> = (0..effective_bytes).map(|i| (i % 256) as u8).collect();
        let seq = self.next_sequence;
        self.next_sequence += 1;
        let mut frame = SensorFrame::new(2, seq, seq, payload);
        frame.dimensions = [samples, 1, 1];
        frame.quality = 1.0;
        self.buffer.push(frame.clone());
        Ok(frame)
    }

    pub fn buffer_len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn dropped_count(&self) -> u64 {
        self.buffer.dropped_count()
    }
}

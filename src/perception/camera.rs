use super::sensor::{BoundedBuffer, DropPolicy, SensorFrame};
use crate::Result;

pub struct CameraConfig {
    pub max_frames: u32,
    pub max_frame_bytes: u32,
    pub drop_policy: DropPolicy,
}

pub struct MockCamera {
    config: CameraConfig,
    buffer: BoundedBuffer<SensorFrame>,
    next_sequence: u64,
    running: bool,
}

impl MockCamera {
    pub fn new(config: CameraConfig) -> Self {
        let buffer = BoundedBuffer::new(config.max_frames as usize, config.drop_policy);
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

    pub fn capture_frame(&mut self, width: u32, height: u32) -> Result<SensorFrame> {
        if !self.running {
            return Err(crate::Error::PerceptionCameraError(
                "camera not running".into(),
            ));
        }
        let pixel_count = (width * height) as usize;
        let max_pixels = (self.config.max_frame_bytes / 4) as usize;
        let effective_pixels = if pixel_count > max_pixels {
            max_pixels
        } else {
            pixel_count
        };
        let payload: Vec<u8> = (0..effective_pixels * 4).map(|i| (i % 256) as u8).collect();
        let seq = self.next_sequence;
        self.next_sequence += 1;
        let mut frame = SensorFrame::new(1, seq, seq, payload);
        frame.dimensions = [width, height, 1];
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

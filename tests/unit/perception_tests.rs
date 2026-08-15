use anr::perception::audio::{AudioConfig, MockAudio};
use anr::perception::camera::{CameraConfig, MockCamera};
use anr::perception::sensor::{BoundedBuffer, DropPolicy, SensorFormat, SensorFrame};

#[test]
fn tc_u_sensor_frame_001() {
    let frame = SensorFrame::new(5, 200, 10, vec![10, 20, 30]);
    assert_eq!(frame.sensor_id, 5);
    assert_eq!(frame.timestamp, 200);
    assert_eq!(frame.sequence, 10);
    assert_eq!(frame.payload, vec![10, 20, 30]);
    assert_eq!(frame.format, SensorFormat::Raw);
}

#[test]
fn tc_u_sensor_frame_002() {
    let mut frame = SensorFrame::new(1, 0, 0, vec![1]);
    frame.dimensions = [100, 100, 1];
    frame.quality = 0.5;
    assert!(frame.is_valid());
}

#[test]
fn tc_u_sensor_frame_003() {
    let mut frame = SensorFrame::new(1, 0, 0, vec![1]);
    frame.dimensions = [100, 100, 1];
    frame.quality = 0.0;
    assert!(!frame.is_valid());
}

#[test]
fn tc_u_sensor_frame_004() {
    let frame = SensorFrame::new(1, 0, 0, vec![0u8; 512]);
    assert_eq!(frame.size_bytes(), 512);
}

#[test]
fn tc_u_sensor_frame_005() {
    let mut buf: BoundedBuffer<u32> = BoundedBuffer::new(4, DropPolicy::DropOldest);
    assert!(buf.push(10));
    assert!(buf.push(20));
    assert_eq!(buf.len(), 2);
}

#[test]
fn tc_u_sensor_frame_006() {
    let mut buf: BoundedBuffer<u32> = BoundedBuffer::new(2, DropPolicy::DropOldest);
    buf.push(1);
    buf.push(2);
    assert!(!buf.push(3));
    assert_eq!(buf.pop(), Some(2));
    assert_eq!(buf.pop(), Some(3));
}

#[test]
fn tc_u_sensor_frame_007() {
    let mut buf: BoundedBuffer<u32> = BoundedBuffer::new(2, DropPolicy::DropNewest);
    buf.push(1);
    buf.push(2);
    assert!(!buf.push(3));
    assert_eq!(buf.pop(), Some(1));
    assert_eq!(buf.pop(), Some(2));
}

#[test]
fn tc_u_sensor_frame_008() {
    let mut buf: BoundedBuffer<u32> = BoundedBuffer::new(2, DropPolicy::DropOldest);
    buf.push(99);
    assert_eq!(buf.pop(), Some(99));
}

#[test]
fn tc_u_sensor_frame_009() {
    let mut buf: BoundedBuffer<u32> = BoundedBuffer::new(2, DropPolicy::DropOldest);
    assert_eq!(buf.pop(), None);
}

#[test]
fn tc_u_sensor_frame_010() {
    let mut buf: BoundedBuffer<u32> = BoundedBuffer::new(1, DropPolicy::DropOldest);
    buf.push(1);
    buf.push(2);
    buf.push(3);
    assert_eq!(buf.dropped_count(), 2);
}

#[test]
fn tc_u_sensor_frame_011() {
    let mut buf: BoundedBuffer<u32> = BoundedBuffer::new(5, DropPolicy::DropOldest);
    buf.push(1);
    buf.push(2);
    buf.clear();
    assert!(buf.is_empty());
}

#[test]
fn tc_u_sensor_frame_012() {
    let mut buf: BoundedBuffer<u32> = BoundedBuffer::new(3, DropPolicy::DropOldest);
    buf.push(1);
    buf.push(2);
    buf.push(3);
    assert!(buf.is_full());
}

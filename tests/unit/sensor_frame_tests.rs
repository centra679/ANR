use anr::perception::sensor::{BoundedBuffer, DropPolicy, SensorFormat, SensorFrame};

#[test]
fn tc_u_sensor_001() {
    let frame = SensorFrame::new(1, 100, 1, vec![1, 2, 3]);
    assert_eq!(frame.sensor_id, 1);
    assert_eq!(frame.timestamp, 100);
    assert_eq!(frame.sequence, 1);
    assert_eq!(frame.payload, vec![1, 2, 3]);
    assert_eq!(frame.format, SensorFormat::Raw);
    assert_eq!(frame.quality, 1.0);
    assert_eq!(frame.flags, 0);
    assert_eq!(frame.dimensions, [0, 0, 0]);
}

#[test]
fn tc_u_sensor_002() {
    let mut frame = SensorFrame::new(1, 100, 1, vec![1, 2, 3]);
    frame.dimensions = [1920, 1080, 3];
    frame.quality = 1.0;
    assert!(frame.is_valid());
}

#[test]
fn tc_u_sensor_003() {
    let mut frame = SensorFrame::new(1, 100, 1, vec![1, 2, 3]);
    frame.dimensions = [1920, 1080, 3];
    frame.quality = 0.0;
    assert!(!frame.is_valid());
}

#[test]
fn tc_u_sensor_004() {
    let payload = vec![0u8; 1024];
    let frame = SensorFrame::new(1, 100, 1, payload);
    assert_eq!(frame.size_bytes(), 1024);
}

#[test]
fn tc_u_sensor_005() {
    let mut buf: BoundedBuffer<u32> = BoundedBuffer::new(3, DropPolicy::DropOldest);
    assert!(buf.push(1));
    assert!(buf.push(2));
    assert!(buf.push(3));
    assert_eq!(buf.len(), 3);
}

#[test]
fn tc_u_sensor_006() {
    let mut buf: BoundedBuffer<u32> = BoundedBuffer::new(3, DropPolicy::DropOldest);
    buf.push(1);
    buf.push(2);
    buf.push(3);
    assert!(!buf.push(4));
    assert_eq!(buf.pop(), Some(2));
    assert_eq!(buf.pop(), Some(3));
    assert_eq!(buf.pop(), Some(4));
    assert_eq!(buf.len(), 0);
}

#[test]
fn tc_u_sensor_007() {
    let mut buf: BoundedBuffer<u32> = BoundedBuffer::new(3, DropPolicy::DropNewest);
    buf.push(1);
    buf.push(2);
    buf.push(3);
    assert!(!buf.push(4));
    assert_eq!(buf.pop(), Some(1));
    assert_eq!(buf.pop(), Some(2));
    assert_eq!(buf.pop(), Some(3));
    assert_eq!(buf.len(), 0);
}

#[test]
fn tc_u_sensor_008() {
    let mut buf: BoundedBuffer<u32> = BoundedBuffer::new(3, DropPolicy::DropOldest);
    buf.push(42);
    assert_eq!(buf.pop(), Some(42));
    assert!(buf.is_empty());
}

#[test]
fn tc_u_sensor_009() {
    let mut buf: BoundedBuffer<u32> = BoundedBuffer::new(3, DropPolicy::DropOldest);
    assert_eq!(buf.pop(), None);
}

#[test]
fn tc_u_sensor_010() {
    let mut buf: BoundedBuffer<u32> = BoundedBuffer::new(2, DropPolicy::DropOldest);
    buf.push(1);
    buf.push(2);
    assert_eq!(buf.dropped_count(), 0);
    buf.push(3);
    assert_eq!(buf.dropped_count(), 1);
    buf.push(4);
    assert_eq!(buf.dropped_count(), 2);
}

#[test]
fn tc_u_sensor_011() {
    let mut buf: BoundedBuffer<u32> = BoundedBuffer::new(3, DropPolicy::DropOldest);
    buf.push(1);
    buf.push(2);
    buf.push(3);
    buf.clear();
    assert!(buf.is_empty());
    assert_eq!(buf.len(), 0);
}

#[test]
fn tc_u_sensor_012() {
    let mut buf: BoundedBuffer<u32> = BoundedBuffer::new(2, DropPolicy::DropOldest);
    buf.push(1);
    buf.push(2);
    assert!(buf.is_full());
}

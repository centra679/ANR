use anr::perception::fusion::{Fusion, PerceptionOutput};
use anr::perception::sensor::{SensorFormat, SensorFrame};
use anr::perception::Perception;

#[test]
fn tc_u_fusion_001() {
    let fusion = Fusion::new(8);
    let output = fusion.fuse(&[]);
    assert_eq!(output.fused_features.len(), 8);
}

#[test]
fn tc_u_fusion_002() {
    let fusion = Fusion::new(4);
    let mut frame = SensorFrame::new(1, 100, 0, vec![255, 0, 128, 64]);
    frame.dimensions = [2, 2, 1];
    frame.quality = 1.0;
    assert!(fusion.validate_frame(&frame));
}

#[test]
fn tc_u_fusion_003() {
    let fusion = Fusion::new(4);
    let mut frame = SensorFrame::new(1, 100, 0, vec![255, 0, 128, 64]);
    frame.dimensions = [2, 2, 1];
    frame.quality = 0.0;
    assert!(!fusion.validate_frame(&frame));
}

#[test]
fn tc_u_fusion_004() {
    let fusion = Fusion::new(4);
    let mut frame = SensorFrame::new(1, 100, 0, vec![128, 64, 32, 16]);
    frame.dimensions = [2, 2, 1];
    frame.quality = 0.8;
    let output = fusion.fuse(&[frame]);
    assert_eq!(output.sensor_count, 1);
    assert_eq!(output.fused_features.len(), 4);
}

#[test]
fn tc_u_fusion_005() {
    let fusion = Fusion::new(2);
    let mut f1 = SensorFrame::new(1, 100, 0, vec![255, 0]);
    f1.dimensions = [1, 1, 1];
    f1.quality = 1.0;
    let mut f2 = SensorFrame::new(2, 200, 0, vec![128, 64]);
    f2.dimensions = [1, 1, 1];
    f2.quality = 1.0;
    let output = fusion.fuse(&[f1, f2]);
    assert_eq!(output.sensor_count, 2);
}

#[test]
fn tc_u_fusion_006() {
    let fusion = Fusion::new(2);
    let mut f1 = SensorFrame::new(1, 100, 0, vec![128, 64]);
    f1.dimensions = [1, 1, 1];
    f1.quality = 1.0;
    let mut f2 = SensorFrame::new(2, 200, 0, vec![32, 16]);
    f2.dimensions = [1, 1, 1];
    f2.quality = 1.0;
    let output = fusion.fuse(&[f1, f2]);
    assert_eq!(output.timestamp, 200);
}

#[test]
fn tc_u_fusion_007() {
    let fusion = Fusion::new(2);
    let mut f1 = SensorFrame::new(1, 100, 0, vec![128, 64]);
    f1.dimensions = [1, 1, 1];
    f1.quality = 1.0;
    let mut f2 = SensorFrame::new(2, 200, 0, vec![32, 16]);
    f2.dimensions = [1, 1, 1];
    f2.quality = 1.0;
    let mut f3 = SensorFrame::new(3, 300, 0, vec![8, 4]);
    f3.dimensions = [1, 1, 1];
    f3.quality = 1.0;
    let output = fusion.fuse(&[f1, f2, f3]);
    assert_eq!(output.sensor_count, 3);
}

#[test]
fn tc_u_fusion_008() {
    let fusion = Fusion::new(2);
    let mut f1 = SensorFrame::new(1, 100, 0, vec![128, 64]);
    f1.dimensions = [1, 1, 1];
    f1.quality = 0.9;
    let output = fusion.fuse(&[f1]);
    assert!(output.confidence > 0.0);
}

#[test]
fn tc_u_fusion_009() {
    let p = Perception::new(16);
    let frames = p.process_frames(&[]);
    assert!(frames.is_ok());
}

#[test]
fn tc_u_fusion_010() {
    let p = Perception::new(4);
    let mut f1 = SensorFrame::new(1, 100, 0, vec![255, 128, 64, 32]);
    f1.dimensions = [2, 2, 1];
    f1.quality = 1.0;
    let output = p.process_frames(&[f1]).unwrap();
    assert_eq!(output.sensor_count, 1);
    assert_eq!(output.fused_features.len(), 4);
}

#[test]
fn tc_u_fusion_011() {
    let p = Perception::new(4);
    let output = p.process_frames(&[]).unwrap();
    assert_eq!(output.sensor_count, 0);
    assert_eq!(output.confidence, 0.0);
    assert_eq!(output.fused_features.len(), 4);
}

#[test]
fn tc_u_fusion_012() {
    let fusion = Fusion::new(8);
    let mut frame = SensorFrame::new(1, 100, 0, vec![10; 32]);
    frame.dimensions = [4, 4, 2];
    frame.quality = 1.0;
    let output = fusion.fuse(&[frame]);
    assert_eq!(output.fused_features.len(), 8);
}

#[cfg(test)]
mod soa_layout_tests {
    use anr::neural::block::BlockPool;
    use anr::neural::cell::{CellPool, CellState};
    use anr::neural::column::ColumnPool;
    use anr::neural::synapse::SynapseTable;

    #[test]
    fn tc_u_soa_001() {
        let pool = CellPool::new(16);
        assert_eq!(pool.ids.len(), 16);
        assert_eq!(pool.activation.len(), 16);
        assert_eq!(pool.potential.len(), 16);
        assert_eq!(pool.threshold.len(), 16);
        assert_eq!(pool.state.len(), 16);
        assert_eq!(pool.refractory_until.len(), 16);
        assert_eq!(pool.last_fired.len(), 16);
        assert_eq!(pool.usage.len(), 16);
    }

    #[test]
    fn tc_u_soa_002() {
        let pool = ColumnPool::new(8);
        assert_eq!(pool.ids.len(), 8);
        assert_eq!(pool.cell_start.len(), 8);
        assert_eq!(pool.cell_len.len(), 8);
        assert_eq!(pool.winner_idx.len(), 8);
        assert_eq!(pool.state.len(), 8);
        assert_eq!(pool.inhibition.len(), 8);
        assert_eq!(pool.usage.len(), 8);
    }

    #[test]
    fn tc_u_soa_003() {
        let pool = BlockPool::new(4);
        assert_eq!(pool.block_id.len(), 4);
        assert_eq!(pool.context_tag.len(), 4);
        assert_eq!(pool.column_set_offset.len(), 4);
        assert_eq!(pool.column_set_len.len(), 4);
        assert_eq!(pool.temporal_depth.len(), 4);
        assert_eq!(pool.prediction_score.len(), 4);
        assert_eq!(pool.state.len(), 4);
    }

    #[test]
    fn tc_u_soa_004() {
        let table = SynapseTable::new(32);
        assert_eq!(table.source_kind.len(), 32);
        assert_eq!(table.source_id.len(), 32);
        assert_eq!(table.target_kind.len(), 32);
        assert_eq!(table.target_id.len(), 32);
        assert_eq!(table.weight.len(), 32);
        assert_eq!(table.strength.len(), 32);
        assert_eq!(table.state.len(), 32);
        assert_eq!(table.last_active.len(), 32);
        assert_eq!(table.age.len(), 32);
        assert_eq!(table.plasticity.len(), 32);
    }

    #[test]
    fn tc_u_soa_005() {
        let mut pool = CellPool::new(4);
        pool.ids[2] = 42;
        pool.activation[2] = 0.75;
        pool.potential[2] = 0.9;
        pool.threshold[2] = 0.5;
        pool.state[2] = CellState::Integrating;
        pool.refractory_until[2] = 5;
        pool.last_fired[2] = 3;
        let cell = pool.get(2);
        assert_eq!(cell.id, 42);
        assert!((cell.activation - 0.75).abs() < 1e-6);
        assert!((cell.potential - 0.9).abs() < 1e-6);
        assert!((cell.threshold - 0.5).abs() < 1e-6);
        assert_eq!(cell.state, CellState::Integrating);
        assert_eq!(cell.refractory_until, 5);
        assert_eq!(cell.last_fired, 3);
    }

    #[test]
    fn tc_u_soa_006() {
        let mut pool = ColumnPool::new(4);
        pool.ids[1] = 7;
        pool.state[1] = anr::neural::column::ColumnState::Winner;
        pool.inhibition[1] = 0.9;
        pool.cell_start[1] = 0;
        pool.cell_len[1] = 3;
        assert_eq!(pool.ids[1], 7);
        assert_eq!(pool.state[1], anr::neural::column::ColumnState::Winner);
        assert!((pool.inhibition[1] - 0.9).abs() < 1e-6);
    }

    #[test]
    fn tc_u_soa_007() {
        let mut pool = BlockPool::new(2);
        pool.block_id[0] = 10;
        pool.context_tag[0] = 20;
        pool.prediction_score[0] = 0.8;
        let block = pool.get(0).unwrap();
        assert_eq!(block.id, 10);
        assert_eq!(block.context_id, 20);
        assert!((block.prediction_state - 0.8).abs() < 1e-6);
    }

    #[test]
    fn tc_u_soa_008() {
        let mut table = SynapseTable::new(0);
        let idx = table.add(0, 10, 1, 20);
        let syn = table.get(idx).unwrap();
        assert_eq!(syn.source, 10);
        assert_eq!(syn.target, 20);
        assert!((syn.weight - 0.5).abs() < 1e-6);
        assert!((syn.permanence - 0.5).abs() < 1e-6);
    }

    #[test]
    fn tc_u_soa_009() {
        let pool = CellPool::new(256);
        assert_eq!(pool.capacity(), 256);
    }

    #[test]
    fn tc_u_soa_010() {
        let mut pool = CellPool::new(8);
        for i in 0..8 {
            pool.ids[i] = i as u32;
            pool.usage[i] = 1;
        }
        let input_currents = vec![1.0f32; 8];
        let fired = pool.update_all(0, &input_currents);
        for &idx in &fired {
            assert!((idx as usize) < 8);
        }
    }

    #[test]
    fn tc_u_soa_011() {
        let mut pool = CellPool::new(8);
        pool.activation[0] = 0.5;
        pool.potential[0] = 0.3;
        pool.state[0] = CellState::Firing;
        pool.reset_all();
        for i in 0..8 {
            assert!((pool.activation[i]).abs() < 1e-6);
            assert!((pool.potential[i]).abs() < 1e-6);
            assert_eq!(pool.state[i], CellState::Resting);
        }
    }

    #[test]
    fn tc_u_soa_012() {
        let pool = CellPool::new(64);
        assert_eq!(pool.ids.capacity(), 64);
        assert_eq!(pool.activation.capacity(), 64);
        assert_eq!(pool.potential.capacity(), 64);
        assert_eq!(pool.threshold.capacity(), 64);
        assert_eq!(pool.state.capacity(), 64);
    }
}

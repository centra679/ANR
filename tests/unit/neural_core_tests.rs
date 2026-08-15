#[cfg(test)]
mod neural_core_tests {
    use anr::neural::column::ColumnState;
    use anr::neural::synapse::Synapse;
    use anr::neural::NeuralCore;
    use anr::simd::SimdBackend;

    #[test]
    fn tc_u_neural_core_001() {
        let core = NeuralCore::new(32, 4, 8, 64);
        assert_eq!(core.cell_pool.capacity(), 32);
        assert_eq!(core.column_pool.capacity(), 4);
        assert_eq!(core.block_pool.capacity(), 8);
        assert_eq!(core.synapse_table.capacity(), 64);
    }

    #[test]
    fn tc_u_neural_core_002() {
        let mut core = NeuralCore::new(32, 4, 8, 64);
        for i in 0..32 {
            core.cell_pool.ids[i] = i as u32;
            core.cell_pool.usage[i] = 1;
        }
        let input_currents = vec![1.0f32; 32];
        let fired = core.cycle(0, &input_currents);
        assert!(!fired.is_empty());
    }

    #[test]
    fn tc_u_neural_core_003() {
        let mut core = NeuralCore::new(32, 4, 8, 64);
        for i in 0..32 {
            core.cell_pool.usage[i] = 1;
            core.cell_pool.threshold[i] = 0.1;
        }
        let col_cap = core.column_pool.capacity();
        for i in 0..col_cap {
            core.column_pool.usage[i] = 1;
            core.column_pool.cell_start[i] = (i * 8) as u32;
            core.column_pool.cell_len[i] = 8;
        }
        let input = vec![1.0f32; 32];
        core.cycle(0, &input);
        let active = core.active_columns();
        for &col_id in &active {
            assert!((col_id as usize) < col_cap);
        }
    }

    #[test]
    fn tc_u_neural_core_004() {
        let core = NeuralCore::new(32, 4, 8, 64);
        assert!(matches!(
            core.backend(),
            SimdBackend::Scalar | SimdBackend::Neon | SimdBackend::Avx2
        ));
    }

    #[test]
    fn tc_u_neural_core_005() {
        let mut core = NeuralCore::new(32, 4, 8, 64);
        for i in 0..32 {
            core.cell_pool.usage[i] = 1;
        }
        let input = vec![0.0f32; 32];
        let fired = core.cycle(0, &input);
        assert!(fired.is_empty());
    }

    #[test]
    fn tc_u_neural_core_006() {
        let mut core = NeuralCore::new(32, 4, 8, 64);
        for i in 0..32 {
            core.cell_pool.usage[i] = 1;
        }
        let input = vec![10.0f32; 32];
        let fired = core.cycle(0, &input);
        assert!(!fired.is_empty());
    }

    #[test]
    fn tc_u_neural_core_007() {
        let mut core = NeuralCore::new(32, 4, 8, 64);
        for i in 0..32 {
            core.cell_pool.usage[i] = 1;
        }
        let input = vec![1.0f32; 32];
        let _f1 = core.cycle(0, &input);
        let _f2 = core.cycle(1, &input);
        let _f3 = core.cycle(2, &input);
        assert!(core.cell_pool.last_fired.iter().any(|&f| f > 0));
    }

    #[test]
    fn tc_u_neural_core_008() {
        let mut core = NeuralCore::new(32, 4, 8, 64);
        let syn = Synapse::new(1, 0, 1);
        core.graph.add_synapse(syn);
        assert_eq!(core.graph.synapse_count(), 1);
    }

    #[test]
    fn tc_u_neural_core_009() {
        let mut core = NeuralCore::new(32, 4, 8, 64);
        let mut syn1 = Synapse::new(1, 0, 1);
        syn1.permanence = 0.8;
        let mut syn2 = Synapse::new(2, 0, 2);
        syn2.permanence = 0.05;
        core.graph.add_synapse(syn1);
        core.graph.add_synapse(syn2);
        core.graph.prune_weak_synapses(0.1);
        assert_eq!(core.graph.synapse_count(), 1);
    }

    #[test]
    fn tc_u_neural_core_010() {
        let mut core = NeuralCore::new(32, 4, 8, 64);
        assert_eq!(core.graph.synapse_count(), 0);
        core.graph.add_synapse(Synapse::new(1, 0, 1));
        assert_eq!(core.graph.synapse_count(), 1);
        core.graph.add_synapse(Synapse::new(2, 0, 2));
        assert_eq!(core.graph.synapse_count(), 2);
    }

    #[test]
    fn tc_u_neural_core_011() {
        let core = NeuralCore::default();
        assert!(core.cell_pool.capacity() > 0);
        assert!(core.column_pool.capacity() > 0);
        assert!(core.block_pool.capacity() > 0);
        assert!(core.synapse_table.capacity() > 0);
    }

    #[test]
    fn tc_u_neural_core_012() {
        let core = NeuralCore::new(64, 16, 8, 128);
        let cell_cap = core.cell_pool.capacity();
        let col_cap = core.column_pool.capacity();
        let mut col_pool = anr::neural::column::ColumnPool::new(col_cap);
        for i in 0..col_cap {
            col_pool.usage[i] = 1;
            col_pool.cell_start[i] = (i * (cell_cap / col_cap)) as u32;
            col_pool.cell_len[i] = (cell_cap / col_cap) as u32;
        }
        assert_eq!(cell_cap, 64);
        assert_eq!(col_cap, 16);
    }
}

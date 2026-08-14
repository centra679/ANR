/// Neural Core Unit Tests
#[cfg(test)]
mod neural_cell_state {
    use anr::neural::Cell;

    #[test]
    fn tc_u_cell_state_001() {
        let cell = Cell::new_with_threshold(0, 0.5);
        assert_eq!(cell.id, 0);
    }

    #[test]
    fn tc_u_cell_state_002() {
        let cell = Cell::new_with_threshold(0, 0.5);
        assert_eq!(cell.potential, 0.0);
    }

    #[test]
    fn tc_u_cell_state_003() {
        let cell = Cell::new_with_threshold(0, 0.5);
        assert!(!cell.is_firing());
    }

    #[test]
    fn tc_u_cell_state_004() {
        let cell = Cell::new_with_threshold(0, 0.5);
        assert!(!cell.is_refractory());
    }

    #[test]
    fn tc_u_cell_state_005() {
        let mut cell = Cell::new_with_threshold(0, 0.5);
        cell.potential = 1.0;
        cell.fire();
        assert!(cell.is_firing());
    }

    #[test]
    fn tc_u_cell_state_006() {
        let mut cell = Cell::new_with_threshold(0, 0.5);
        cell.potential = 0.3;
        cell.fire();
        assert!(cell.is_firing());
    }

    #[test]
    fn tc_u_cell_state_007() {
        let mut cell = Cell::new_with_threshold(0, 0.5);
        cell.potential = 2.0;
        cell.update(0, 0.0);
        assert!(cell.refractory_until > 0);
    }

    #[test]
    fn tc_u_cell_state_008() {
        let mut cell = Cell::new_with_threshold(0, 0.5);
        cell.update_simple();
        assert!(cell.activation <= 0.0);
    }

    #[test]
    fn tc_u_cell_state_009() {
        let mut cell = Cell::new_with_threshold(0, 0.5);
        let old_potential = cell.potential;
        cell.update_simple();
        assert_eq!(cell.potential, old_potential);
    }

    #[test]
    fn tc_u_cell_state_010() {
        let cell = Cell::new_with_threshold(u32::MAX, 0.5);
        assert_eq!(cell.id, u32::MAX);
    }

    #[test]
    fn tc_u_cell_state_011() {
        let mut cell = Cell::new_with_threshold(0, 0.5);
        for _ in 0..10 {
            cell.update_simple();
        }
        assert!(cell.activation < 0.5);
    }

    #[test]
    fn tc_u_cell_state_012() {
        let mut cell = Cell::new_with_threshold(0, 0.5);
        cell.threshold = 0.0;
        cell.potential = 0.1;
        cell.fire();
        assert!(cell.is_firing());
    }
}

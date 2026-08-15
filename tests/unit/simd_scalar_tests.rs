#[cfg(test)]
mod simd_scalar_tests {
    use anr::simd::scalar::ScalarKernel;
    use anr::simd::{detect_backend, get_kernel, SimdBackend, SimdKernel};

    #[test]
    fn tc_u_scalar_001() {
        let kernel = ScalarKernel;
        let input = [0.0f32];
        let mut output = [0.0f32; 1];
        kernel.activate_sigmoid(&input, &mut output);
        assert!((output[0] - 0.5).abs() < 1e-6);
    }

    #[test]
    fn tc_u_scalar_002() {
        let kernel = ScalarKernel;
        let input = [100.0f32];
        let mut output = [0.0f32; 1];
        kernel.activate_sigmoid(&input, &mut output);
        assert!(output[0] > 0.99);
    }

    #[test]
    fn tc_u_scalar_003() {
        let kernel = ScalarKernel;
        let input = [-100.0f32];
        let mut output = [0.0f32; 1];
        kernel.activate_sigmoid(&input, &mut output);
        assert!(output[0] < 0.01);
    }

    #[test]
    fn tc_u_scalar_004() {
        let kernel = ScalarKernel;
        let input = [5.0f32];
        let mut output = [0.0f32; 1];
        kernel.activate_relu(&input, &mut output);
        assert_eq!(output[0], 5.0);
    }

    #[test]
    fn tc_u_scalar_005() {
        let kernel = ScalarKernel;
        let input = [-3.0f32];
        let mut output = [0.0f32; 1];
        kernel.activate_relu(&input, &mut output);
        assert_eq!(output[0], 0.0);
    }

    #[test]
    fn tc_u_scalar_006() {
        let kernel = ScalarKernel;
        let input = [0.0f32];
        let mut output = [0.0f32; 1];
        kernel.activate_relu(&input, &mut output);
        assert_eq!(output[0], 0.0);
    }

    #[test]
    fn tc_u_scalar_007() {
        let kernel = ScalarKernel;
        let a = [1.0f32, 2.0, 3.0];
        let b = [4.0f32, 5.0, 6.0];
        assert!((kernel.dot_product(&a, &b) - 32.0).abs() < 1e-6);
    }

    #[test]
    fn tc_u_scalar_008() {
        let kernel = ScalarKernel;
        let a: [f32; 0] = [];
        let b: [f32; 0] = [];
        assert!((kernel.dot_product(&a, &b)).abs() < 1e-6);
    }

    #[test]
    fn tc_u_scalar_009() {
        let kernel = ScalarKernel;
        let weights = [0.5f32, 0.5];
        let activations = [1.0f32, 1.0];
        let mut out = [0.0f32; 2];
        kernel.weighted_accumulate(&weights, &activations, &mut out);
        assert!((out[0] - 0.5).abs() < 1e-6);
        assert!((out[1] - 0.5).abs() < 1e-6);
    }

    #[test]
    fn tc_u_scalar_010() {
        let kernel = ScalarKernel;
        let values = [1.0f32, 3.0, 2.0];
        assert_eq!(kernel.argmax(&values), 1);
    }

    #[test]
    fn tc_u_scalar_011() {
        let kernel = ScalarKernel;
        let values = [42.0f32];
        assert_eq!(kernel.argmax(&values), 0);
    }

    #[test]
    fn tc_u_scalar_012() {
        let backend = detect_backend();
        let _kernel = get_kernel(backend);
        assert!(matches!(
            backend,
            SimdBackend::Scalar | SimdBackend::Neon | SimdBackend::Avx2
        ));
    }
}

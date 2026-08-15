pub mod avx;
pub mod neon;
pub mod scalar;

/// SIMD backend selector (SD-13)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimdBackend {
    Scalar,
    Neon,
    Avx2,
}

/// Trait for SIMD-accelerated neural kernels
pub trait SimdKernel {
    fn backend(&self) -> SimdBackend;

    fn activate_sigmoid(&self, input: &[f32], output: &mut [f32]);

    fn activate_relu(&self, input: &[f32], output: &mut [f32]);

    fn dot_product(&self, a: &[f32], b: &[f32]) -> f32;

    fn weighted_accumulate(&self, weights: &[f32], activations: &[f32], out: &mut [f32]);

    fn argmax(&self, values: &[f32]) -> usize;
}

pub fn detect_backend() -> SimdBackend {
    #[cfg(target_arch = "aarch64")]
    {
        SimdBackend::Neon
    }
    #[cfg(target_arch = "x86_64")]
    {
        if std::arch::is_x86_feature_detected!("avx2") {
            SimdBackend::Avx2
        } else {
            SimdBackend::Scalar
        }
    }
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    {
        SimdBackend::Scalar
    }
}

pub fn get_kernel(backend: SimdBackend) -> Box<dyn SimdKernel> {
    match backend {
        SimdBackend::Scalar => Box::new(scalar::ScalarKernel),
        SimdBackend::Neon => Box::new(scalar::ScalarKernel),
        SimdBackend::Avx2 => Box::new(scalar::ScalarKernel),
    }
}

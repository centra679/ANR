use super::{SimdBackend, SimdKernel};

pub struct ScalarKernel;

impl SimdKernel for ScalarKernel {
    fn backend(&self) -> SimdBackend {
        SimdBackend::Scalar
    }

    fn activate_sigmoid(&self, input: &[f32], output: &mut [f32]) {
        for (i, &x) in input.iter().enumerate() {
            output[i] = 1.0 / (1.0 + (-x).exp());
        }
    }

    fn activate_relu(&self, input: &[f32], output: &mut [f32]) {
        for (i, &x) in input.iter().enumerate() {
            output[i] = if x > 0.0 { x } else { 0.0 };
        }
    }

    fn dot_product(&self, a: &[f32], b: &[f32]) -> f32 {
        a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
    }

    fn weighted_accumulate(&self, weights: &[f32], activations: &[f32], out: &mut [f32]) {
        for (i, (w, a)) in weights.iter().zip(activations.iter()).enumerate() {
            if i < out.len() {
                out[i] += w * a;
            }
        }
    }

    fn argmax(&self, values: &[f32]) -> usize {
        values
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
}

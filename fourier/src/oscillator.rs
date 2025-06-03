use std::f32::consts::PI;

use nannou::glam::Vec2;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

use crate::complex::Complex;

#[derive(Debug, Clone, Copy)]
pub struct Oscillator {
    phase: f32,
    frequency: f32,
    amplitude: f32,
}

impl Oscillator {
    pub fn update(&mut self, dt: f32) {
        if self.frequency == 0.0 {
            return;
        }
        self.phase = (self.phase + dt * self.frequency) % (2.0 * std::f32::consts::PI);
    }

    pub fn as_vector(&self) -> Vec2 {
        Vec2::new(
            self.amplitude * self.phase.cos(),
            self.amplitude * self.phase.sin(),
        )
    }

    pub fn phase(&self) -> f32 {
        self.phase
    }

    pub fn frequency(&self) -> f32 {
        self.frequency
    }

    pub fn amplitude(&self) -> f32 {
        self.amplitude
    }
}


pub fn oscillators_from_complex(complex: &Vec<Complex>) -> Vec<Oscillator> {
    if complex.len() == 0 {
        return vec![];
    } else if complex.len() == 1 {
        return vec![Oscillator {
            phase: complex[0].argument(),
            frequency: 0.0,
            amplitude: complex[0].magnitude(),
        }];
    }

    let n = complex.len() as i32;

    (0..n).into_par_iter().map(|freq| {
        let frequency = if freq <= n/2 { 
            freq 
        } else { 
            freq - n 
        };
        
        let c_n:Complex = (0..complex.len()).into_par_iter().fold(|| Complex::zero(), |acc, k| {
            // DFT formula: negative exponent
            let angle = -2.0 * PI * (frequency as f32) * (k as f32) / (n as f32);
            let exponential = Complex::new(angle.cos(), angle.sin());
            acc + complex[k] * exponential
        }).sum::<Complex>() / (n as f32);
        
        Oscillator {
            phase: c_n.argument(),
            frequency: frequency as f32,
            amplitude: c_n.magnitude(),
        }
    }).collect()
}

// Test function to verify reconstruction
pub fn test_reconstruction(complex: &Vec<Complex>, oscillators: &Vec<Oscillator>) {
    println!("Testing reconstruction:");
    let n = complex.len();
    
    for k in 0..n {
        let t = 2.0 * PI * (k as f32) / (n as f32);
        let mut reconstructed = Complex::zero();
        
        for osc in oscillators {
            let angle = osc.frequency * t + osc.phase;
            let contribution = Complex::new(
                osc.amplitude * angle.cos(), 
                osc.amplitude * angle.sin()
            );
            reconstructed = reconstructed + contribution;
        }
        
        println!("Original[{}]: ({:.3}, {:.3}), Reconstructed: ({:.3}, {:.3})", 
                 k, complex[k].re(), complex[k].im(), reconstructed.re(), reconstructed.im());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::complex::Complex;

    #[test]
    fn test_oscillator_reconstruction() {
        let complex = vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 1.0),
            Complex::new(-1.0, 0.0),
            Complex::new(0.0, -1.0),
        ];
        
        let oscillators = oscillators_from_complex(&complex);
        test_reconstruction(&complex, &oscillators);
    }
}
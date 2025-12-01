use crate::{
    audio::{AudioSource, Sample},
    core::Frequency,
};

pub struct SineOscillator {
    frequency: Frequency,
}

impl SineOscillator {
    pub fn new(frequency: Frequency) -> Self {
        Self { frequency }
    }
}

impl SineOscillator {
    pub fn render<T: From<f32>>(&mut self) -> T {
        0.0.into()
    }
}

/// Implementing [`AudioSource`] for the oscillator allows the
/// oscillator to be used directly as a source in an audio chain.
impl<T: Sample> AudioSource<T> for SineOscillator {
    fn render(&mut self, buffer: &'_ mut crate::audio::Buffer<T>) {
        todo!()
    }
}

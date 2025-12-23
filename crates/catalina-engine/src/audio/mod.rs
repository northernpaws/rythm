// Traits for working with audio samples.
pub mod sample;
pub use sample::{FromSample, Sample};

// Traits for working with audio frames, one or
// more samples based on the sampling rate.
pub mod frame;
pub use frame::{Frame, Mono, Stereo};

// Traits and methods for working with slices of samples and frames.
pub mod slice;

// Traits and implementations for working with oscillators.
pub mod oscillator;

pub trait AudioSource {
    type Frame: Frame;

    /// Render a buffered block of audio from the audio source.
    fn render(&mut self, buffer: &'_ mut [Self::Frame]);
}

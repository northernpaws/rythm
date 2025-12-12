pub mod oscillator;

pub use dasp::{
    Frame, Sample,
    frame::{self, Mono, Stereo},
    sample::FromSample,
    slice,
};

pub trait AudioSource {
    type Frame: Frame;

    /// Render a buffered block of audio from the audio source.
    fn render(&mut self, buffer: &'_ mut [Self::Frame]);
}

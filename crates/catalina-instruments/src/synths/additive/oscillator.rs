use catalina_engine::{
    audio::{FromSample, Sample, oscillator},
    core::Hertz,
    music::note::Note,
};

/// Implements the oscillators for the additive synth, including parameters
/// for per-oscillator phase shifting and frequency adjustments.
pub(crate) struct AdditiveOscillator {
    enabled: bool,
    /// Base frequency of the oscillator.
    base_frequency: Hertz,

    /// Specifies if the frequency of the ocillator is fixed to
    /// the base frequency, or can slide depending on the note.
    ///
    /// NOTE: There is an opportunity here later to add an optimzation
    ///  by using an oscillator lookup table for fixed frequency.
    fixed_frequency: bool,

    /// The amplitude level in the range 0..1 for the oscillator.
    level: f32,
}

impl AdditiveOscillator {
    pub fn new(enabled: bool, base_frequency: Hertz) -> Self {
        Self {
            enabled,
            base_frequency,
            fixed_frequency: false,
            level: 1.0,
        }
    }

    /// Returns if the oscillator is enabled.
    #[inline]
    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }

    #[inline]
    pub const fn base_frequency(&self) -> Hertz {
        self.base_frequency
    }

    /// Calculates the frequency that should be used
    /// for the oscillator given the specified note.
    #[inline]
    pub fn note_frequency(&self, note: &'_ Note) -> Hertz {
        // If we're using a fixed frequency, then we don't
        // apply an offset based on the played note.
        if self.fixed_frequency {
            return self.base_frequency;
        }

        // Get the frequency of the note in hertz.
        //
        // We use this as the base frequency of our oscillators so
        // that the oscillator plays in-key with the triggered note.
        let note_freq = note.frequency();

        // Relatively offset the base frequency based on the played note.
        let offset_freq = self.base_frequency - note_freq;

        self.base_frequency + offset_freq
    }

    /// Sample the oscillator with the provided phase.
    ///
    /// The phase passed here is derived from the phase maintained in each voice.
    pub fn sample<S: Sample + FromSample<f32>>(&self, phase: f32) -> S {
        (oscillator::sine::<f32>(phase) * self.level).to_sample()
    }
}

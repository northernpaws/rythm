/// A voice renders the output sound from the synth.
///
/// In a monophonic synth there is a single voice that
/// is usually taken over by the most recent keypress.
///
/// In polyphonic synths there are several voices that
/// can play sounds from multiple keys at once.
pub(crate) struct Voice {
    /// Phase of the voice to be fed to the oscillators.
    ///
    /// Note that because the speed of the phase change is
    /// relative to the frequency and sample rate, we need
    /// to maintain a seperate phase for each of the 4
    /// oscillators because they may each be set to a
    /// different base frequency.
    ///
    /// Increments each sample, and loops back
    /// to 0 when exceeding the sample rate.
    pub(crate) phase_0: f32,
    pub(crate) phase_1: f32,
    pub(crate) phase_2: f32,
    pub(crate) phase_3: f32,
}

impl Voice {
    /// Constructs a new voice for the additive synth.
    pub fn new() -> Self {
        Self {
            phase_0: 0.0,
            phase_1: 0.0,
            phase_2: 0.0,
            phase_3: 0.0,
        }
    }
}

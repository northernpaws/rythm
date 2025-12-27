use heapless::index_map::FnvIndexMap;

use catalina_engine::{
    audio::{AudioSource, signal::Signal},
    instrument::{Instrument, NoteError},
    music::note::{self, Note},
};

pub mod oscillator;
pub(crate) use oscillator::AdditiveOscillator;

pub mod voice;
pub(crate) use voice::Voice;

/// A type of synthesizer that adds multiple oscillators together, typically sine
/// waves, at different frequencies, amplitudes and phases to build harmonics.
pub struct AdditiveSynth {
    sample_rate: usize,

    /// We have a bank of 4 optional oscillators that are added for each voice.
    ///
    /// At least the first oscillator needs to be enabled, the rest are optional.
    oscillators: [AdditiveOscillator; 4],

    /// Configure the instrument with 8-voice polyphony.
    ///
    /// Each voice pair tracks the phase data for that note.
    voices: FnvIndexMap<Note, Voice, 8>,
}

impl AdditiveSynth {
    /// Construct a new instance of the additive synth.
    pub fn new(sample_rate: usize) -> Self {
        Self {
            sample_rate,

            // By default we're only populating the first oscillator.
            oscillators: [
                AdditiveOscillator::new(true, note::CFour.frequency()),
                AdditiveOscillator::new(false, note::CFour.frequency()),
                AdditiveOscillator::new(false, note::CFour.frequency()),
                AdditiveOscillator::new(false, note::CFour.frequency()),
            ],

            voices: FnvIndexMap::new(),
        }
    }
}

/// The interfaces for controlling the instrument from the framework.
impl Instrument for AdditiveSynth {
    fn init(&mut self) {}

    /// Called when a note is pressed.
    fn note_on(&mut self, note: Note, _velocity: u8) -> Result<(), NoteError> {
        // Attempt to add a voice.
        //
        // .insert() will return an error if the voices map is full.
        self.voices
            .insert(
                note,         // This is the note we're adding a voice for
                Voice::new(), // This holds the data for the voice.
            )
            .map_err(|_| NoteError::NoVoices)?;

        // There should ideally be some logic here to prempt
        // voices, but that's an exercise for later.

        Ok(())
    }

    /// Called when a note is released.
    fn note_off(&mut self, note: Note) {
        // Remove the voice for the note when the note is released.
        self.voices.remove(&note);
    }
}

/// Allows the synth to be used in [`Signal`]` chains.
impl Signal for AdditiveSynth {
    type Frame = f32;

    /// Produces the next frame of audio from the synth.
    fn next(&mut self) -> Self::Frame {
        // The final sample for the frame.
        //
        // This is the result of all the voices (active notes) summed together.
        let mut sample = 0.0;

        // Loop through each active voice and sum them for the frame.
        for (note, voice) in self.voices.iter_mut() {
            // The sample for this voice.
            //
            // This is the result of the oscillators summed
            // together (the add in **add**itive synthesis).
            let mut voice_sample = 0.0;

            // Process the first oscillator for the voice, if enabled.
            if self.oscillators[0].is_enabled() {
                let osc = &self.oscillators[0];
                // Sample each configured oscillator and add them together.
                voice_sample = voice_sample + osc.sample::<f32>(voice.phase_0);

                // Shift the base oscillator phase of the voice
                // so that the voices oscillate independently.
                voice.phase_0 =
                    voice.phase_0 + (osc.note_frequency(note).hertz() / self.sample_rate as f32);
                if voice.phase_0 >= 1.0 {
                    voice.phase_0 = 0.0;
                }
            }

            // Process the second oscillator for the voice, if enabled.
            if self.oscillators[1].is_enabled() {
                let osc = &self.oscillators[1];
                // Sample each configured oscillator and add them together.
                voice_sample = voice_sample + osc.sample::<f32>(voice.phase_1);

                // Shift the base oscillator phase of the voice
                // so that the voices oscillate independently.
                voice.phase_1 =
                    voice.phase_1 + (osc.note_frequency(note).hertz() / self.sample_rate as f32);
                if voice.phase_1 >= 1.0 {
                    voice.phase_1 = 0.0;
                }
            }

            // Process the third oscillator for the voice, if enabled.
            if self.oscillators[2].is_enabled() {
                let osc = &self.oscillators[2];
                // Sample each configured oscillator and add them together.
                voice_sample = voice_sample + osc.sample::<f32>(voice.phase_2);

                // Shift the base oscillator phase of the voice
                // so that the voices oscillate independently.
                voice.phase_2 =
                    voice.phase_2 + (osc.note_frequency(note).hertz() / self.sample_rate as f32);
                if voice.phase_2 >= 1.0 {
                    voice.phase_2 = 0.0;
                }
            }

            // Process the fourth oscillator for the voice, if enabled.
            if self.oscillators[3].is_enabled() {
                let osc = &self.oscillators[3];
                // Sample each configured oscillator and add them together.
                voice_sample = voice_sample + osc.sample::<f32>(voice.phase_3);

                // Shift the base oscillator phase of the voice
                // so that the voices oscillate independently.
                voice.phase_3 =
                    voice.phase_3 + (osc.note_frequency(note).hertz() / self.sample_rate as f32);
                if voice.phase_3 >= 1.0 {
                    voice.phase_3 = 0.0;
                }
            }

            sample = sample + voice_sample;
        }

        // Note that the resulting buffer will be clipped on playback
        // depending on the voice count and frequencies.
        //
        // It's on the receiving end of the rendered buffer to apply
        // amplitude scaling to bring the audio samples down to an
        // acceptable level for playback.
        sample
    }
}

impl AudioSource for AdditiveSynth {
    type Frame = f32;

    fn render(&mut self, buffer: &'_ mut [Self::Frame]) {
        for i in 0..buffer.len() {
            // Note that the resulting buffer will be clipped on playback
            // depending on the voice count and frequencies.
            //
            // It's on the receiving end of the rendered buffer to apply
            // amplitude scaling to bring the audio samples down to an
            // acceptable level for playback.
            buffer[i] = self.next();
        }
    }
}

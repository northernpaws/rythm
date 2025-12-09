use heapless::index_map::FnvIndexMap;

use rythm_engine::{
    audio::{AudioSource, Buffer, Sample, oscillator::SineOscillator},
    instrument::{Instrument, NoteError},
    theory::note::Note,
};

struct Voice {
    /// The sine oscillator used to render the voice.
    pub osc: SineOscillator,

    /// A per-voice timebase for the oscillator index to allow each voice
    /// to oscillate relative to when the trigger key was pressed.
    pub time: usize,
}

/// Example instrument implementation that just plays a sine wave ocillator.
pub struct SineInstrument {
    /// Configure the instrument with 8-voice polyphony.
    ///
    /// Since we're a basic sine synth, we use one
    /// sine wave oscillator as each synth voice.
    voices: FnvIndexMap<Note, Voice, 8>,
}

impl SineInstrument {
    pub fn new() -> Self {
        Self {
            voices: FnvIndexMap::new(),
        }
    }
}

impl<T: Sample> AudioSource<T> for SineInstrument {
    fn render(&mut self, buffer: &'_ mut Buffer<T>) {
        for i in 0..buffer.frames() {
            let mut frame: [f32; 8] = [0_f32; 8];

            // Loop through each active voice and sum it to the output buffer.
            let mut j = 0;
            for (_, mut voice) in self.voices.iter() {
                // TODO: need to feed proper sample time base
                frame[j] = voice.render(voice.time);
                j += 1;

                // Make sure to increment the sine time index so the oscillator.. oscillates
                voice.time = (voice.time + 1) & voice.osc.get_sample_rate();
            }
        }
    }
}

impl<T: Sample> Instrument<T> for SineInstrument {
    fn init(&mut self) {}

    fn note_on(&mut self, note: Note, velocity: u8) -> Result<(), NoteError> {
        // Get the frequency of the note in hertz.
        let freq = note.frequency();

        // Attempt to add a voice.
        //
        // .insert() will return an error if the voices map is full.
        self.voices
            .insert(
                note,
                Voice {
                    // Feed the note frequency to a sine oscillator.
                    osc: SineOscillator::new(freq, 44100),
                    time: 0,
                },
            )
            .map_err(|_| NoteError::NoVoices)?;

        Ok(())
    }

    fn note_off(&mut self, note: Note) {
        // Remove the voice for the note when the note is released.
        self.voices.remove(&note);
    }
}

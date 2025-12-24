//! Implements the common attack, decay, sustain and release
//! (ADSR) envelope used by most audio synthesis.

/// Derrived from the C++ constant.
const M_E: f32 = 2.71828182845904523536;

#[derive(PartialEq, Eq)]
pub enum EnvelopeStage {
    Init,
    Attack,
    Decay,
    Release,
}

/// Implements the common attack, decay, sustain and release
/// (ADSR) envelope used by most audio synthesis.
///
/// - Attack specifies the amount of time and the slope for going from no audio, to the peak of the source.
/// - Decay is the rolloff from the attack peak to the sustain level.
/// - Sustain specifies the level the sound is maintained at as long as the envelope is triggered.
/// - Release is the rolloff from sustain to no sound
///
/// Attack, decay and release specify the time and rolloff for
/// their stages, while sustain specifies the sustain level.
///
/// This ADSR implementation was originally written by Paul Batchelor for Soundpipe.
/// In May of 2020 it was then ported from Soundpipe by Ben Sergentanis for DaisyDSP,
/// and eventually remade by Steffan DIedrichsen in May of 2021. It was then adapted
/// to Rust on December 2025 with several improvements by Kat Mitchell for Catalina.
pub struct Envelope {
    /// The sample rate the audio engine is being ran at.
    sample_rate: usize,

    /// The time it takes the envelope to go from silent to it's peak level.
    attack_time: f32,
    /// The level the sound is raised to at attack, percentage from 0.0 to 1.0.
    attack_level: f32,
    /// The time it takes to go from the peak level to the sustain level.
    decay_time: f32,
    /// The level the sound is sustained at, percentage from 0.0 to 1.0.
    sustain_level: f32,
    /// The time it takes the sound to return to silence after release.
    release_time: f32,

    /// Used in the attack time coefficiant calculation.
    ///
    /// Cached as a field to prevent needlessly re-calculating the attack coefficiant.
    attack_shape: f32,

    /// Attack coeff
    attack_d0: f32,
    /// Decay coeff
    decay_d0: f32,
    /// Release coeff
    release_d0: f32,

    /// The stage the envelope is currently at.
    stage: EnvelopeStage,
    /// The currently known state of the gate signal.
    ///
    /// When this changes to true, it triggers the envelope's attack
    /// stage. When changed to false, it transitions to the release.
    gate: bool,
    x: f32,
}

impl Envelope {
    pub fn new(sample_rate: usize) -> Self {
        let mut adsr = Self {
            sample_rate,

            attack_time: -1.0,
            attack_level: 0.0,
            decay_time: -1.0,
            sustain_level: 0.0,
            release_time: -1.0,

            attack_shape: -1.0,

            attack_d0: 0.0,
            decay_d0: 0.0,
            release_d0: 0.0,
            stage: EnvelopeStage::Init,
            gate: false,
            x: 0.0,
        };

        adsr.set_attack_time(0.1, 0.0);
        adsr.set_decay_time(0.1);
        adsr.set_release_time(0.1);

        adsr
    }

    /// Configures the attack time ramp for the ADSR envelope.
    pub fn set_attack_time(&mut self, seconds: f32, shape: f32) {
        if (seconds != self.attack_time) || (shape != self.attack_shape) {
            self.attack_time = seconds;
            self.attack_shape = shape;

            if seconds > 0.0 {
                let x: f32 = shape;
                let target: f32 = 9.0 * libm::powf(x, 10.0) + 0.3 * x + 1.01;
                self.attack_level = target;
                let log_target: f32 = libm::logf(1.0 - (1.0 / target)); // -1 for decay
                self.attack_d0 = 1.0 - libm::expf(log_target / (seconds * self.sample_rate as f32));
            } else {
                self.attack_d0 = 1.0; // instant change
            }
        }
    }

    /// Sets the duration of the decay part of the envelope, when
    /// transitioning from the attack peak to the sustain level.
    pub fn set_decay_time(&mut self, seconds: f32) {
        if seconds != self.decay_time {
            self.decay_time = seconds;
            if self.decay_time > 0.0 {
                let target: f32 = libm::logf(1. / M_E);
                self.decay_d0 =
                    1.0 - libm::expf(target / (self.decay_time * self.sample_rate as f32));
            } else {
                self.decay_d0 = 1.0; // instant change
            }
        }
    }

    /// Sets the duration of the release stage of the envelope, when
    /// the key is released and the envelope is transitioning from
    /// the sustatin level to silence.
    pub fn set_release_time(&mut self, seconds: f32) {
        if seconds != self.release_time {
            self.release_time = seconds;
            if self.release_time > 0.0 {
                let target: f32 = libm::logf(1. / M_E);
                self.release_d0 =
                    1.0 - libm::expf(target / (self.release_time * self.sample_rate as f32));
            } else {
                self.release_d0 = 1.0; // instant change
            }
        }
    }

    /// Sets the sustain level from 0.0 to 1.0.
    pub fn set_sustain_level(&mut self, level: f32) {
        // Make sure the sustain level is clamped from 0.0 to 1.0
        if level <= 0.0 {
            self.sustain_level = -0.01;
        } else if level > 1.0 {
            self.sustain_level = 1.0;
        } else {
            self.sustain_level = level;
        }
    }

    /// Processes a single sample from the envelope.
    ///
    /// The returned float is a percentage of the current level of the envelope.
    /// Multiply this by a sound source to apply the envelope to it.
    ///
    /// Gate triggers the envelope when true, and starts the decay/release
    /// when false. This is typically tied to a note press/release
    pub fn process(&mut self, gate: bool) -> f32 {
        // When the incoming gate signal is true and the local one
        // is false, that means we're seeing a rising edge and the
        // attack stage should be triggered.
        if gate && !self.gate {
            self.stage = EnvelopeStage::Attack;
        } else if !gate && self.gate {
            // We're seeing a falling gate signal, and
            // should trigger the release stage.
            self.stage = EnvelopeStage::Release;
        }

        // Determine which coefficiant to use depending
        // on the current stage of the envelope.
        let d0 = if self.stage == EnvelopeStage::Decay {
            self.decay_d0
        } else if self.stage == EnvelopeStage::Release {
            self.release_d0
        } else {
            self.attack_d0
        };

        let mut out: f32;

        match self.stage {
            EnvelopeStage::Init => 0.0,
            EnvelopeStage::Attack => {
                self.x += d0 * (self.attack_level - self.x);
                out = self.x;
                if out > 1.0 {
                    self.x = 1.0;
                    out = 1.0;
                    self.stage = EnvelopeStage::Decay;
                }

                out
            }
            EnvelopeStage::Decay | EnvelopeStage::Release => {
                // Determine the audio target level based on the current stage.
                let target: f32 = if self.stage == EnvelopeStage::Decay {
                    self.sustain_level
                } else {
                    -0.01
                };

                self.x += d0 * (target - self.x);
                out = self.x;
                if out < 0.0 {
                    self.x = 0.0;
                    out = 0.0;
                    self.stage = EnvelopeStage::Init;
                }

                out
            }
        }
    }
}

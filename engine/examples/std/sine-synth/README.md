# `sine-synth`

An basic example of how to construct an 8-voice polyphonic instrument using oscillators as voices.

> Note that there is a slight crackle when new notes are added, this is due to there being note ADSR (Attack, Decay, Sustain, Release) envelope on the voices. Oscillator synths will typically have at last a small ~5ms attack, decay, and release to prevent this.

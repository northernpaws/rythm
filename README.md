# Rythm

> ⚠️ This is still a large work-in-progress! A semi-working version (hardware and software) is estimated to be ready around March 2026.

Rythm is an embedded-first Rust ecosystem for buiding music and audio processing devices.

The Rythm ecosystem is comprised of:
 * A Rust-based audio engine designed for embedded devices (heapless, small footprint).
 * Two base hardware modules designed for the engine:
   * **Rythm Mini** - small footprint with audio processing.
   * **Rythm Card** - larger footprint with audio processing, CV and sync.
 * Several module "host" boards:
   * Breadboard-compatible carrier with headers.
   * Protoboard-style carrier with an integrated module.

## Engine

The Rythm engine is an embedded-first audio engine in the form of a Rust crate that can be easily integrated into embedded projects as a dependency, or used 

## Modules

### Mini

  * 480MHz ARM Cortex-M processor
  * 64MB of SDRAM
  * QSPI Flash
  * Stereo Audio Codec
  * SDMMC 4-bit interface
  * Stereo Audio Codec (2 in, 2 out)

#### Notes

 * Mini was originally designed with the BGA201 package, but the pitch was too fine for economic fabrication capabilities. Upgraded to BGA265 in revision 2 to fix routing constraints. 

### Card

Everything from the Mini:

  * 480MHz ARM Cortex-M processor
  * 64MB of SDRAM
  * QSPI Flash
  * Stereo Audio Codec
  * SDMMC 4-bit interface
  * Stereo Audio Codec (2 in, 2 out)

Plus:

  * 1 Hardware MIDI Endpoint (RX and TX)
  * Eurorack x CV inputs
  * Eurorack x CV outputs
  * Eurorack Sync In/Out


## Kits

Rythm's hardware ecosystem will also include several soldering kits that are currently in various stages of development:
 * **Rythm Mini**
   * Guitar/Effects pedal kit.
   * Mini synth kit.
   * Mini step sequencer kit.
 * **Rythm Card**
   * Eurorack module kit.
   * Pro synth kit.
   * Pro step sequencer kit.

Each of these kits will come with a board support package via a feature flag in the Rust crate. These board support packages will come with all the scaffolding necessary to quickly build a project around the boards, as well as several example projects. 

## TODO
  * FUSB303BTMX on discovery for USB-C support
  * Amphenol_12401610E4-2A or HRO_TYPE-C-31-M-12?
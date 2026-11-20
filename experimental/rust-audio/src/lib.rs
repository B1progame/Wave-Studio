//! A small WebAssembly bridge to the original Pro Wave audio engine.
//! This is an experimental building block, not a browser port of the desktop application.
use prowave_engine::{Command, EngineCore, Garbage, Shared};
use rtrb::{Consumer, Producer, RingBuffer};
use std::sync::Arc;
use wasm_bindgen::prelude::*;

/// Owns one engine instance and its command/garbage queues on the calling thread.
#[wasm_bindgen]
pub struct WebAudioEngine {
    core: EngineCore,
    commands: Producer<Command>,
    garbage: Consumer<Garbage>,
}

#[wasm_bindgen]
impl WebAudioEngine {
    /// Construct an engine for the browser audio context's sample rate.
    #[wasm_bindgen(constructor)]
    pub fn new(sample_rate: u32) -> Self {
        let (commands, rx) = RingBuffer::new(256);
        let (trash, garbage) = RingBuffer::new(256);
        let core = EngineCore::new(sample_rate, rx, trash, Arc::new(Shared::new()));
        Self {
            core,
            commands,
            garbage,
        }
    }

    /// Queue a live instrument note. Returns false when the command queue is full.
    pub fn note_on(&mut self, pitch: u8, velocity: u8, track: u16) -> bool {
        self.commands
            .push(Command::NoteOn {
                pitch,
                vel: velocity,
                track,
            })
            .is_ok()
    }

    /// Release a live note on a track. Returns false when the queue is full.
    pub fn note_off(&mut self, pitch: u8, track: u16) -> bool {
        self.commands
            .push(Command::NoteOffTrack { pitch, track })
            .is_ok()
    }

    /// Set the stereo output gain (the engine validates and smooths the value).
    pub fn set_master_gain(&mut self, gain: f32) -> bool {
        self.commands
            .push(Command::SetMasterGain { linear: gain })
            .is_ok()
    }

    /// Render interleaved stereo frames. Limit allocation to one second per call.
    pub fn render(&mut self, frames: usize) -> Vec<f32> {
        let mut output = vec![0.0; frames.min(48_000) * 2];
        self.core.process(&mut output);
        while self.garbage.pop().is_ok() {}
        output
    }
}

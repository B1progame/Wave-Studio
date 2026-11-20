# Experimental Rust audio bridge

This is an early WebAssembly binding to the original GPL-3.0-or-later Pro Wave audio engine. It queues live note-on/note-off and master gain commands and renders interleaved stereo samples. It is not wired into the website and is not a complete port. The current render method allocates; it is not suitable for a real-time AudioWorklet callback without redesign.

Install Rust and the wasm32-unknown-unknown target, then run `cargo check --target wasm32-unknown-unknown`. The original Pro Wave source is fetched from GitHub at the revision pinned in Cargo.toml. Browser audio integration, project data, file handling, the egui UI, plug-ins, and many desktop-only facilities still need porting and testing.

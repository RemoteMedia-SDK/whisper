//! Whisper as a loadable plugin, backed by Candle on every target.
//!
//! `WhisperNode` is defined in `remotemedia-candle-nodes` using the
//! `#[node(...)]` attribute macro. Adding `loadable-export` to this crate's
//! dep-features list flips the macro's conditional emission, which:
//!
//! * compiles a `WhisperNodeLoadableFactory` unit struct implementing
//!   `FfiNodeFactory`;
//! * registers a `LoadableFactoryEntry { make: ... }` via `inventory`;
//! * keeps everything else (the in-tree `AsyncStreamingNode` impl, the
//!   `WhisperNodeFactory` consumed by `CandleNodesProvider`) intact.
//!
//! All this crate has to do is call the arg-less `plugin_export!()` macro —
//! it walks the inventory at startup and emits the abi_stable root module the
//! host loads via `dlopen`.
//!
//! Candle is pure Rust, so the same node builds for Android. The LiteRT
//! Whisper nodes ship in their own plugin (`litert-nodes`).

// Force the linker to keep the upstream `remotemedia-candle-nodes` rlib's
// object files and therefore the `inventory::submit!` static the `#[node]`
// macro emitted for `WhisperNode`.
#[allow(dead_code)]
fn _force_link_whisper_factory() {
    let _f = remotemedia_candle_nodes::whisper::WhisperNodeFactory::new();
}

remotemedia_plugin_sdk::plugin_export!();

//! Whisper as a loadable plugin (Phase B / Path 2 — dual-emit).
//!
//! `WhisperNode` is defined in `remotemedia-candle-nodes` using the
//! `#[node(...)]` attribute macro. Adding `loadable-export` to this
//! crate's dep-features list flips the macro's conditional emission,
//! which:
//!
//! * compiles a `WhisperNodeLoadableFactory` unit struct implementing
//!   `FfiNodeFactory`;
//! * registers a `LoadableFactoryEntry { make: ... }` via `inventory`;
//! * keeps everything else (the in-tree `AsyncStreamingNode` impl, the
//!   `WhisperNodeFactory` consumed by `CandleNodesProvider`) intact.
//!
//! All this crate has to do is call the arg-less `plugin_export!()`
//! macro — it walks the inventory at startup and emits the abi_stable
//! root module the host loads via `dlopen`. Same shape as
//! `examples/loadable-export-smoke/`, but the in-tree node is real
//! (Whisper + Candle) instead of a 5-line echo.

// Force the linker to keep the upstream `remotemedia-candle-nodes`
// rlib's object files (and therefore the
// `inventory::submit!{ LoadableFactoryEntry { ... } }` static the
// `#[node]` macro emitted for `WhisperNode`). Without a hard
// reference into the rlib, the linker prunes it as dead code and
// the inventory comes up empty at runtime — hence
// `plugin exposes: []` from the host.
//
// Touching any pub symbol from candle-nodes is enough; we reach for
// the most-stable thing on the surface.
#[allow(dead_code)]
fn _force_link_whisper_factory() {
    let _f = remotemedia_candle_nodes::whisper::WhisperNodeFactory::new();
}

remotemedia_plugin_sdk::plugin_export!();

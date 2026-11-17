# whisper — Whisper STT as a RemoteMedia SDK loadable plugin

Single-file Rust cdylib that registers `WhisperNode` into the
[RemoteMedia SDK](https://github.com/matbeedotcom/remotemedia-sdk)
streaming pipeline registry via the
[`#[node(loadable_export)]`](https://github.com/matbeedotcom/remotemedia-sdk/blob/main/docs/CUSTOM_NODE_REGISTRATION.md)
dual-emit path (Path 2 / 4). Backed by candle-rs.

## Use from a manifest

```json
{
  "version": "v1",
  "plugins": ["whisper@v0.3.0"],
  "nodes": [
    {
      "id": "stt",
      "node_type": "Whisper",
      "params": { "model": "tiny", "transcript_updates": { "mode": "full" } }
    }
  ]
}
```

The SDK resolver expands `whisper@v0.3.0` to
`github.com/RemoteMedia-SDK/whisper`, fetches `plugin.toml`, then
falls through to `release-manifest.json` for the platform-specific
prebuilt `.so` / `.dylib` / `.dll` asset.

> **Status:** plugin.toml + source published. **Prebuilt release
> binaries are not yet uploaded** — the matrix-build CI workflow is
> pending. Until then, consumers should either build the cdylib
> themselves (see below) or use a local-path plugin entry.

## Build the cdylib locally

```bash
git clone https://github.com/RemoteMedia-SDK/whisper
cd whisper
cargo build --release
# → target/release/libwhisper_loadable_plugin.so
```

Then reference it from your manifest:

```json
{ "plugins": ["./path/to/libwhisper_loadable_plugin.so"] }
```

## What it exports

| Node type | Input | Output |
|-----------|-------|--------|
| `Whisper` (desktop; Candle) | Audio chunks (any rate and channel count; resampled to 16 kHz mono), `{"type": "flush"}` | Transcript events, or Text |

### Streaming transcript events

With `transcript_updates: { "mode": "full" }` (or `"stable"`), the node
emits the `delta` / `correction` / `final` JSON events that every
RemoteMedia STT node shares (`remotemedia_modality::stt`), decoding a
rolling window of the audio it has received. Send `{"type": "flush"}` to
close the utterance with a `final`. `stable` sends committed text only,
and never corrects it.

`agreement_updates` and `trailing_guard_tokens` in the same object tune
how soon text commits. The window is set with `streamingDecodeIntervalMs`,
`streamingOverlapMs` and `streamingMaxWindowMs`.

Without `transcript_updates`, the output is the same as in 0.2: the
`partial` / `correction` / `final` events with `stable_text` /
`unstable_text` (now deprecated) when `streaming` is on (the default),
and plain Text otherwise.

0.3 is built against `remotemedia-plugin-sdk` `740b847`
(loadable-node-abi 0.11), so it loads only in hosts built at the same
plugin SDK revision.

## What's in the repo

```
whisper/
├── plugin.toml      ← metadata (resolver fetches this first)
├── Cargo.toml       ← git-deps the SDK at a pinned rev
├── src/lib.rs       ← arg-less `plugin_export!()` macro call
├── run.sh           ← local smoke-test driver
└── README.md
```

## License

See `LICENSE.md`. This plugin reuses RemoteMedia SDK source and is
governed by the same RemoteMedia SDK Community License 1.0.

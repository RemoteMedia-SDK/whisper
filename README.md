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
  "plugins": ["whisper@v0.1.0"],
  "nodes": [
    {
      "id": "stt",
      "node_type": "WhisperNode",
      "params": { "model_size": "tiny" }
    }
  ]
}
```

The SDK resolver expands `whisper@v0.1.0` to
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

| Node type    | Input                                  | Output |
|--------------|----------------------------------------|--------|
| `WhisperNode` | Audio (16 kHz mono recommended)        | Text   |

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

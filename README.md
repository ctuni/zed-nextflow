# zed-nextflow

[![Zed Extension](https://img.shields.io/badge/Zed-extension-7C3AED)](https://zed.dev/)
[![Nextflow](https://img.shields.io/badge/Nextflow-language%20support-22B573)](https://www.nextflow.io/)
[![Version](https://img.shields.io/badge/version-0.0.1-blue)](./extension.toml)
[![Rust](https://img.shields.io/badge/Rust-2021-DEA584)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

Nextflow language support for [Zed](https://zed.dev/), powered by:

- The Nextflow language server (`nextflow-io/language-server`)
- Tree-sitter syntax parsing (`nextflow-io/tree-sitter-nextflow`)

This extension provides Nextflow file recognition, syntax highlighting/folding, bracket auto-closing, and LSP-backed editor features.

## Features

- **Language registration** for `*.nf` files and Nextflow shebang scripts
- **Syntax highlighting** via Tree-sitter queries in `languages/nextflow/highlights.scm`
- **Code folding** via Tree-sitter queries in `languages/nextflow/folds.scm`
- **Language server integration** for completions and language intelligence
- **Automatic language server download** on first use:
  - Fetches the latest release from `nextflow-io/language-server`
  - Downloads `language-server-all.jar`
  - Starts it with `java -jar ...`

## Requirements

- [Zed](https://zed.dev/)
- `java` available in your `PATH` (required to run the Nextflow language server JAR)
- Rust toolchain (only needed if you want to build the extension locally)

## Install / Run

### As a local dev extension

1. Clone this repository.
2. Build the extension WebAssembly artifact:

```bash
cargo build --release --target wasm32-wasip2
cp target/wasm32-wasip2/release/my_zed_nextflow.wasm extension.wasm
```

3. In Zed, load this folder as a dev extension (Extensions UI).
4. Open a `.nf` file.

On first activation, the extension downloads `language-server-all.jar` and then launches it with Java.

## Compatibility matrix

| Component | Requirement | Notes |
|---|---|---|
| Zed | Recent stable release | Extension API dependency is `zed_extension_api = 0.7.0` |
| Java | `java` in `PATH` | Required to run `language-server-all.jar` |
| Rust (dev only) | Stable toolchain | Needed to build `extension.wasm` locally |
| Nextflow language server | Latest GitHub release | Downloaded automatically at first run |
| OS | Linux / macOS expected, Windows untested | Runtime command currently uses `/bin/sh -c` |

## Development

### Useful commands

```bash
# Fast compile check for the extension crate
cargo check

# Build production wasm artifact used by Zed
cargo build --release --target wasm32-wasip2

# Copy build artifact to the extension root
cp target/wasm32-wasip2/release/my_zed_nextflow.wasm extension.wasm
```

### Optional native helper binaries

The repository includes helper binaries under `src/bin` that are gated behind the `native-helpers` feature:

```bash
cargo run --features native-helpers --bin debug
cargo run --features native-helpers --bin real_donwloader
```

These binaries are for local debugging/downloading experiments and are not part of the wasm extension runtime.

## Project layout

- `extension.toml` - Zed extension metadata and grammar/LSP wiring
- `src/lib.rs` - Main extension implementation (download + launch language server)
- `languages/nextflow/config.toml` - Language config (suffixes, brackets, comments)
- `languages/nextflow/highlights.scm` - Highlight query rules
- `languages/nextflow/folds.scm` - Folding query rules
- `grammars/nextflow/` - Tree-sitter grammar source checkout

## Notes

- The grammar revision is pinned in `extension.toml`.
- The language server JAR is downloaded from the latest GitHub release and stored in the extension's writable runtime directory.

## Troubleshooting

### `java: command not found`

- Install a JRE/JDK and ensure `java` is available in your shell `PATH`.
- Verify with:

```bash
java -version
```

### Language server does not start

- Confirm network access to GitHub releases (`nextflow-io/language-server`) on first run.
- Rebuild and refresh the wasm artifact:

```bash
cargo build --release --target wasm32-wasip2
cp target/wasm32-wasip2/release/my_zed_nextflow.wasm extension.wasm
```

- Reload the extension in Zed and reopen a `.nf` file.

### "Script could not be formatted because it has syntax errors" on save

The Nextflow language server includes a formatter that Zed invokes on save. When the LSP cannot format a file (e.g. because it reports parse errors), Zed surfaces this warning.

To silence it, add the following to your Zed `settings.json`:

```json
"languages": {
  "Nextflow": {
    "format_on_save": "off"
  }
}
```

Zed's keyboard shortcut to open settings: `cmd+,` (macOS) / `ctrl+,` (Linux/Windows).

### Syntax highlighting looks incomplete

- Confirm the grammar and query files are present:
  - `languages/nextflow/config.toml`
  - `languages/nextflow/highlights.scm`
  - `languages/nextflow/folds.scm`
- Rebuild `extension.wasm`, reload the extension, and reopen the file.

## License

This project is licensed under the MIT License.

See `LICENSE` for details.

## Publishing checklist

- Ensure `extension.wasm` is rebuilt from current sources.
- Validate extension metadata in `extension.toml` (`id`, `version`, `repository`, `description`).
- Smoke-test by opening a `.nf` file in a clean Zed profile and confirming LSP startup/completions.
- Add CI workflow(s) in `.github/workflows/`, then wire status badges in this README.

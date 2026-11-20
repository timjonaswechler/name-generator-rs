# Name Generator

Experimentelle Rust-Bibliothek für validierte Sprachkonfigurationen als Grundlage einer Namensgenerierung. Anatomie, Phonologie und Silbenstruktur sind getrennte Module. Das Projekt ist unabhängig vom `star_sim`-Workspace.

## Lokal verwenden

Voraussetzungen: Rust mit Cargo und ein C-Compiler für das mitgebaute Lua.

Im Projektverzeichnis:

```sh
cargo check --locked
cargo test --locked
cargo run --locked --example lang
```

Das Beispiel erstellt und validiert eine Sprachkonfiguration und gibt sie aus; es generiert noch keine Namen.

## Beispiele und Entwürfe

`examples/lang.rs` ist das ausführbare Cargo-Beispiel. Die übrigen Dateien in `examples/` sind übernommene Architekturentwürfe, keine eigenständig kompilierbaren Programme. Deshalb ist die automatische Beispielerkennung in `Cargo.toml` deaktiviert und `lang` explizit registriert.

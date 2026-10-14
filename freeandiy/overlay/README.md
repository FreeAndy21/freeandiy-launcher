<!-- Modified-by: FreeANDIY (Free Andy Agency), 2026. Replaces upstream ArtCraft Launcher's README. -->
<p align="center">
  <img src="assets/app-icon/freeandiy-launcher-1024.png" width="128" alt="FreeANDIY Launcher icon">
</p>

<h1 align="center">FreeANDIY Launcher</h1>

<p align="center">
  <b>Installs, updates and opens the FreeANDIY apps. Native Rust, runs on macOS, Windows and Linux.</b><br>
  Part of <a href="https://freeandiy.freeandy.agency">FreeANDIY</a>, free creative and office apps by Free Andy Agency.
</p>

---

## About

FreeANDIY Launcher lists every FreeANDIY app (Photo, Vector, Film, Light, PDF, Effects, Layout,
Write, Slides, Sheets and Sound), shows which are installed and which have updates, and installs
them for you from their GitHub releases. It is a native egui app; it also keeps itself up to date.

It talks only to GitHub: the release pages of the `FreeAndy21/freeandiy-*` repositories, and
downloads only what those releases publish. There is no account, no telemetry and no other
server.

**FreeANDIY Launcher is based on the open-source project
[ArtCraft Launcher](https://github.com/storytold/craft-launcher) (© ArtCraft Team and
contributors, MIT/Apache-2.0). FreeANDIY is not affiliated with the ArtCraft team.**

## Download

Builds for macOS, Windows and Linux are published on the
[Releases](https://github.com/FreeAndy21/freeandiy-launcher/releases) page as
`freeandiy-launcher-<version>-<os>-<arch>.*`.

## Build from source

Requires Rust (see `rust-version` in `Cargo.toml`).

```sh
cargo run --release -p artcraft-launcher                 # desktop app (binary: freeandiy-launcher)
```

The crate names keep upstream's `artcraft-launcher-*`, so upstream changes merge cleanly. The
developer documentation in `docs/` and `AGENTS.md` is upstream's; upstream's README is kept as
[`docs/upstream-README.md`](docs/upstream-README.md).

## How this fork is made

Everything FreeANDIY changes lives in [`freeandiy/`](freeandiy/): the brand definition
(`branding.toml`, `app.toml`), assets, a handful of small patches and text rules. All other files
are regenerated from upstream by `freeandiy/scripts/regenerate.sh`, so updates from upstream never
conflict. See [`freeandiy/docs/upstream-sync.md`](freeandiy/docs/upstream-sync.md).

Issues with FreeANDIY builds go to
[our tracker](https://github.com/FreeAndy21/freeandiy-launcher/issues), not to upstream.

## License

The code is licensed, like upstream, under either of [MIT](LICENSE-MIT) or
[Apache-2.0](LICENSE-APACHE) at your option. See [`NOTICE`](NOTICE) and [`ATTRIBUTION.md`](ATTRIBUTION.md)
for third-party material.

The FreeANDIY and Free Andy Agency names, the angel Andy, logos and app icons are © Free Andy
Agency and **not** covered by the code license; see
[`freeandiy/assets/brand/LICENSE-brand.txt`](freeandiy/assets/brand/LICENSE-brand.txt).

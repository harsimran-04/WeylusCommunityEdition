# AGENTS.md — Weylus Community Edition

## Build

- **Default `cargo build` compiles ffmpeg + libx264 from source** via `deps/build.sh` on the first run. This is slow.
- **To skip the ffmpeg build**, install system ffmpeg with libx264 support and use `cargo build --features ffmpeg-system`. This is recommended for active development.
- **`tsc` must be installed locally** via `npm install typescript@4.9.5 --save-dev`. `build.rs` calls it (via `cmd /c tsc` on Windows, `./node_modules/.bin/tsc` elsewhere) and aborts if it fails. It only re-runs when `ts/lib.ts` is newer than `www/static/lib.js`.
  - **Note:** Use TypeScript 4.9.5 or similar older version. TypeScript 6.x + has breaking changes that cause compilation errors.
- **Linux deps** (Debian/Ubuntu example in `Readme.md`): X11 dev libs, `autoconf`, `libtool-bin`, `nasm`, `nvidia-cuda-dev`, `pkg-config`, `libdrm-dev`, `libpango1.0-dev`, `libgstreamer1.0-dev`, `libgstreamer-plugins-base1.0-dev`, `libdbus-1-dev`.
- **Release** is built with `cargo build --release`. `lto = true` and `opt-level = 3` in `Cargo.toml`.

## Project Layout

| Directory / File | Purpose |
| ---------------- | ------- |
| `src/`           | Rust application (web server, GUI, websocket, video pipeline, input backends) |
| `lib/`           | C helpers: `encode_video.c`, `error.c`, `log.c`, plus Linux-specific X11/uinput code in `lib/linux/` |
| `ts/lib.ts`      | Sole TypeScript source; compiles to `www/static/lib.js` |
| `www/`           | Static web assets served at runtime |
| `deps/`          | External build scripts and ffmpeg build tree; `deps/dist` (or `deps/dist_<os>`) holds the static libs if built locally |

## Features

- `ffmpeg-system` — link against system ffmpeg instead of building from source.
- `va-static` — link `libva` statically on Linux (used for releases).
- `bench` — enables benchmarking code.

## Linux Runtime Requirements

- **Stylus, pressure, and multi-touch need `/dev/uinput` writable.** Udev rules and group setup are documented in `Readme.md`. Without this, the generic mouse-only backend is used.
- **Wayland support is experimental** and requires `pipewire` + `xdg-desktop-portal` (plus the backend specific to your compositor). Known gaps: input mapping for windows, proper window names, cursor capture.
- **VAAPI** hardware acceleration is off by default. Device selection via `WEYLUS_VAAPI_DEVICE` (e.g. `/dev/dri/renderD129`); driver search path via `LIBVA_DRIVERS_PATH`; driver name via `LIBVA_DRIVER_NAME`.

## Cross-Compiling / Release Packaging

- `docker_build.sh` — cross-compiles Windows (`x86_64-pc-windows-gnu`), builds `.deb` via `cargo deb -- --features=va-static`, and assembles release zips.
- `build_in_local_container.sh` — runs the above inside `podman` containers (`hhmhh/weylus_build` for deb/Windows, `hhmhh/weylus_build_alpine` for a musl/Linux binary). Produces `packages/`.
- `cargo deb` (from `cargo-deb`) generates the Debian package.

## Logging / Debugging

- Set `WEYLUS_LOG_LEVEL` to `DEBUG` or `TRACE`.
- Set `WEYLUS_LOG_JSON=true` for structured logs (useful for automation scripts that parse the log).

## Community Edition Context

- Forked from `H-M-H/Weylus`. Additional patches merged via `patch_community_edition.sh`:
  - `electronstudio2/build-fixes`
  - `electronstudio2/community-edition-patches`
  - `lyonbot/pr1` (macOS stylus pressure)
- The script resolves merge conflicts in `.github/workflows/build.yml` and `build.rs` automatically; inspect those files if patch history seems odd.

## Environment Variables for Build

- `I_AM_BUILDING_THIS_AT_HOME_AND_WANT_LIBNPP=y` — adds NVIDIA Performance Primitives support to the custom ffmpeg build.

## Notes

- On Linux, the `deps/dist` directory (or `deps/dist_linux`) is the sentinel: if it exists, `build.rs` will **not** rebuild ffmpeg.
- Windows builds require MSVC as the C compiler; Linux-to-Windows cross-compilation is done with MinGW.

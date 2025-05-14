# `ascot web-controller`

## Building

To build this crate with a `debug` profile run:

```console
cargo build
```

To build this crate with a `release` profile which enables all time and
memory optimizations run:

```console
cargo build --release
```

To build without `logging` feature

```console
cargo build --no-default-features
```

To build with errors and messages in `Italian` language

```console
cargo build --features italian
```

## Tasks

- [ ] Find and present a single device running in the same local network
- [ ] Print route information on command line for logging purposes
- [ ] Run the tool at startup and save the controller address in browser
bookmarks

### Controller


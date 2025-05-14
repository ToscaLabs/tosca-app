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
- [ ] Define a database to save discovered devices
- [ ] Add a function to translate all hazards in Italian

### Web

- Add spinner to notify that the app is searching devices
    - [Spinner types](https://cssloaders.github.io/)
    - [Button with loading spinner in HTML and CSS](https://www.google.com/search?q=how%20to%20add%20a%20loader%20when%20a%20button%20is%20clicked&ie=utf-8&oe=utf-8&client=firefox-b-m)

### Controller


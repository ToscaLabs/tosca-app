# `tosca-app`

[![LICENSE][license badge]][license]

**tosca-app** is a web app for managing
[tosca](https://github.com/ToscaLabs/tosca/) devices.

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

## Web Tasks

### Graphics

- [ ] Add spinner to notify that the app is searching devices (just to tell the user that is waiting for devices to be discovered)
    - [Spinner types](https://cssloaders.github.io/)
    - [Button with loading spinner in HTML and CSS](https://www.google.com/search?q=how%20to%20add%20a%20loader%20when%20a%20button%20is%20clicked&ie=utf-8&oe=utf-8&client=firefox-b-m)
- [ ] Improve error page CSS and HTML
- [ ] Test if the entire web app is responsive
- [ ] Add the temperature icon and `EventSource`
- [ ] Add hazards and modals (look at the `old-work` branch and adapt)
- [ ] Add privacy page to block a determined request according to its hazards

### Usage

- [ ] Use SSE as much as possible to avoid sending the entire page to the server and back when a request is done to a device. It's slow and bad.
- [ ] Save a device state in some way (i.e. if the light state is false, and the incoming event state is false, do nothing. The state is not changed)
- [ ] Check if there are some problems in using the web app, everything should be smoother as much as possible
- [ ] Filone 2 di Smartotum
- [ ] Parse hazards for a device, extracts all their descriptions, and save this data in an array in order to print their content on a page
- [ ] Implement the backend for the privacy page, allowing or blocking hazards through the `tosca-controller`.

<!-- Links -->
[license]: https://github.com/ToscaLabs/tosca-app/blob/master/LICENSE

<!-- Badges -->
[license badge]: https://img.shields.io/badge/license-MIT-blue.svg

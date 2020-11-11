# Lyricer

Lyricer is an addon for waybar to display lyrics.

## Features

1. Read media using `mpris`

2. Fast and lightweight (<0.1% cpu usage, 1.9M after stripping)

3. Completely compatiable with waybar

## Installation

Use `cargo` to build and install it.

```bash
cargo install lyricer
```
or

```bash
cargo build --release
```

## Usage

Add following lines to your `waybar` configuration:

```json
"modules-center": ["custom/lyrics"],
"custom/lyrics": {
    "format": "♪ {}",
    "interval": 1, 
    "exec": "/usr/bin/cat /tmp/lyrics", 
    "exec-if": "test -f /tmp/lyrics",
    "return-type": "json"
}
```

And don't forget to start `lyricer` in the background, preferrably with sway configutation.

## Why it's too laggy?

Contrast to common implementation, `lyricer` will stay idle whenever it can. This means that the lyrics will not change untill they "suppose" to change according to the lrc file. Thus, when user manually change the audio, `lyricer` will not change untill the current lyrics line is finished.

Sometime the lyric will also lag regardless user interaction. The reason behind this is being investigated. PRs or helpful issues are welcome.

## TODOs

The following features are either planning or currently unable to archive:

1. Control media (pause, resume, next, previous) with buttons

2. Colorful output

3. More lyrics support (less error-prone)

4. Fix laggy performance

5. Manual selection of lyric file

## Contributing

Pull requests are welcome.

## License

[GPL3](https://choosealicense.com/licenses/gpl-3.0)
# Lyricer

Lyricer is an addon for waybar to display lyrics.

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

## Contributing

Pull requests are welcome.

## License

[GPL3](https://choosealicense.com/licenses/gpl-3.0)
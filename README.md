# Emoji Picker

Simple emoji picker using egui. Outputs the chosen emoji on stdout.

## Example Wayland Usage

You can use the following script for Wayland integration:

```bash
#!/bin/sh
wtype '' && (emojipicker-rs | wtype -) || (emojipicker-rs | wl-copy)
```

## License

AGPL-3.0-or-later

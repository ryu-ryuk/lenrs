# lenrs – A TUI OCR Snippet Tool for Wayland 

**lenrs** lets you select any screen area, extract text using Tesseract OCR, and auto-copy it to your clipboard.

## Features 
- Uses `slurp` + `grim` to select and capture screen area
- Extracts text using `tesseract`
- Copies text directly to Wayland clipboard

## Dependencies ( ･×･)

Make sure these are installed:
```sh
sudo pacman -S tesseract wl-clipboard grim slurp wtype
yay -S cliphist
```
--- 
## Usage 

---
## Build 
```sh
cargo build --release
```
> Binary will be in `target/release/lenrs`

---
## Contributing (ᇴ‿ฺᇴ)
PRs and Issues are welcome! 

---
## License

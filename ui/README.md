# Badgemagic-RS GUI
A simple app that uses badgemagic-rs under the hood and provides a graphical user interface with [Tauri](https://tauri.app/).  
  
  
If you want to start the app in development mode, make sure you have [Rust](https://www.rust-lang.org/) and [Bun](https://bun.sh/) installed and then simply run
```sh
bun tauri dev
```

Currently implemented:
- Text transfer with
  - Speed,
  - Animation selection,
  - Effects (including somewhat janky text inversion) and
  - 2 different font sizes

TODO:
- [x] Multi-message support
- [ ] Save/Import/Export messages
  - [ ] Persistent Message Store with https://v2.tauri.app/plugin/sql/
  - [ ] Export as TOML
- [ ] Custom Drawing
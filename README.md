# win-say

win-say is a compact Windows implementation of the MacOS [`say`](https://ss64.com/mac/say.html) command.

https://github.com/user-attachments/assets/b776fc05-5c86-4f73-9758-8fe0fd0f3240

## Usage

```bash
cargo run -- "Bro, did you see the latest deck builder roguelite tower defense tetris mazing game? It's kind of sick"

cargo build --release
# add /target/release/win-say.exe to your PATH
win-say "Uhh, I just bought it and deleted my weekend, so.. thanks for that I guess"
```

## Credits
Under the hood, win-say uses the [Windows Runtime](https://en.wikipedia.org/wiki/Windows_Runtime) COM based ABI via `windows-rs`.

Also, [tts-rs](https://github.com/ndarilek/tts-rs) - for their example code.

# 🌤️ Cli-ma
 
This is my **first time trying out Rust** (hehe) — I wanted to test it by building something practical and fun.  
Rust turned out to be really nice for making CLI stuff
It fetches real-time weather data from [wttr.in](https://wttr.in/) and displays it with colors, emojis, and a neat little spinner animation ✨.  

---

## ✨ Features
- 🎨 Rich colored output with emojis  
- ⏳ Animated loading spinner  
- 🌍 City as command-line argument  
- 📊 Shows temperature, feels-like, condition, and humidity  
- ⚡ Fast, lightweight, single-binary app  
- 🦀 My little experiment to learn Rust, polished into a tool  

---

## 🚀 Installation & Running

### Prerequisites
- Install [Rust](https://www.rust-lang.org/tools/install) (make sure `cargo` works).

### Clone the repo
```bash
git clone https://github.com/akash-kamat/clima.git
cd clima
cargo run -- "city-name"
```

To build:
```bash
cargo build --release
```
The executable will be in ```target/release/clima.exe``` (for windows)

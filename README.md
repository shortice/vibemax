# Vibemax on Rust 🦀

Idea from [this project](https://github.com/abyesilyurt/vibesort).

But for only selfhosted LLM and this repo implement max function instead of sort from original repo. Make fun! :D

I tested it on qwen3 0.6B only.

# How run? 
* Clone this repo
```sh
git clone https://github.com/shortice/vibemax
```
* Install llama.cpp;
* Exec: 
```sh
llama-server -hf ggml-org/Qwen3-0.6B-GGUF -hff Qwen3-0.6B-Q8_0.gguf -ngl 99
```
* Next:
```sh
cargo run
```
* Have fun!

# Run tests

```sh
cargo test
```

# Note
This is my first project on Rust.

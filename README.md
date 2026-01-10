# Recho
Recho is a fully local AI chatbot with voice support. It implements a speech-to-speech pipeline based on open-source models like Whisper (speech to text), Piper (speech to text) and a local language model at its core.

Specifically, it uses whisper.cpp via `whisper-rs` and `piper-rs` that uses onnxruntime (ort) under-the-hood.

I built this project primarily to improve my Rust skills. This isn't a robust application, nothing fancy, but it works.

## Demo
[recho_v1_demo.webm](https://github.com/user-attachments/assets/de9396d6-e485-450b-9ebf-464ff67d66bb)

## Building and Running

### Requirements

* Rust (stable)
Install via [rustup](https://rustup.rs)
* Any LLM inference engine with OpenAI-like API (like Ollama, llama.cpp, etc.)

### Building

Clone the repository and build the project using Cargo:

```bash
cargo build
```

## Usage

### Pre-requisites
- An LLM running via inference engines like Ollama, llama.cpp, etc.
- A [Whisper model](https://huggingface.co/ggerganov/whisper.cpp) (like `whisper-ggml-small.bin`)
- A [Piper TTS model](https://huggingface.co/rhasspy/piper-voices) (like `en_US-arctic-medium.onnx` and `en_US-arctic-medium.onnx.json`)

Run the project with:

```bash
cargo run -- -l "http://localhost:1234/v1/chat/completions" -p "piper/en_GB-cori-medium.onnx.json" -w "whisper/ggml-small.bin"
```

*Tested on a Linux system with LMStudio for serving LLMs.*

## Credits
- thewh1teagle's [piper-rs](https://github.com/thewh1teagle/piper-rs)
- tazz4843's [whisper-rs](https://codeberg.org/tazz4843/whisper-rs)

# Recho
Recho is a fully local AI chatbot with voice support. It implements a speech-to-speech pipeline based on open-source models like Whisper (speech to text), Supertonic (speech to text) and a local language model at its core.

Specifically, it uses whisper.cpp via `whisper-rs` and a custom Supertonic implementation that uses onnxruntime (ort) under-the-hood.

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
- A [Supertoic TTS ONNX models and voice styles](https://huggingface.co/Supertone/supertonic-2)

Run the project like this:

```bash
cargo run -- -l "http://localhost:1234/v1/chat/completions" -o "supertonic/models" -v "supertonic/voices/M1.json" -w "whisper/ggml-small.bin"
```

*Tested on a Linux system with LMStudio for serving LLMs.*

## Credits
- thewh1teagle's [piper-rs](https://github.com/thewh1teagle/piper-rs)
- supertone-inc's [supertonic](https://github.com/supertone-inc/supertonic)

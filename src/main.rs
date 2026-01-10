mod tts;
mod stt;
mod llm;
use tts::{TTSModel};
use stt::{AudioRecorder, STTModel};
use llm::{LLM};
use colored::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut llm_client = LLM::new("http://localhost:1234/v1/chat/completions");
    let mut recorder = AudioRecorder::new();
    let mut speech_recogniser = STTModel::new("whisper/ggml-small.bin");
    let mut piper_tts = TTSModel::new("piper/en_US-arctic-medium.onnx.json", 12);

    println!("{}", "This is a speech-to-speech voice assistant.".bright_white().bold());

    loop  {

        println!("{}", "\n(Press the ENTER key to record your voice)".yellow());

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        println!("{}", "Now RECORDING - Say something! To end conversation, say 'BYE'.\n(Press the ENTER key again to stop recording)".bright_red());
        recorder.start_recording().unwrap();

        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).unwrap();

        let (samples, source_sample_rate) = recorder.stop_recording();
        println!("Captured {} samples", samples.len());

        let transcribed_text = speech_recogniser.transcribe(samples, source_sample_rate);

        println!("{}: {}", "User".bold().bright_cyan(), transcribed_text.bright_cyan());

        let chat_response = llm_client.chat(transcribed_text.trim());

        let ai_response = chat_response.unwrap();

        println!("{}: {}", "Assistant".bold().bright_magenta(), ai_response.bright_magenta());

        piper_tts.process_text(&ai_response);

        if transcribed_text.to_lowercase().contains("bye") {
            break;
        }

    }

    Ok(())
}

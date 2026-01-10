mod tts;
mod stt;
mod llm;
use tts::{TTSModel};
use stt::{AudioRecorder, STTModel};
use llm::{LLM};
use colored::*;
use clap::Parser;

/// Speech-to-speech AI assistant built with Rust, Whisper and Piper
#[derive(Parser)]
struct Cli {
    /// The LLM inference engine server's URL
    #[arg(short)]
    llm_url: String,
    /// The path to the Piper model for text-to-speech
    #[arg(short)]
    piper_model_path: std::path::PathBuf,
    /// The specific speaker id if multiple speakers exist
    #[arg(short, default_value_t = 1)]
    speaker_id: u8,
    /// The path to the Whisper model for speech-to-text
    #[arg(short)]
    whisper_model_path: std::path::PathBuf
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Cli::parse();

    let mut recorder = AudioRecorder::new();
    let mut llm_client = LLM::new(&args.llm_url);
    let mut speech_recogniser = STTModel::new(args.whisper_model_path);
    let mut piper_tts = TTSModel::new(args.piper_model_path, args.speaker_id);

    println!("{}", "This is a speech-to-speech voice assistant.".bright_white().bold());

    loop  {

        println!("{}", "[Recording paused]: (Press the ENTER key to record your voice)".yellow());

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        println!("{}", "[Recording started]: Say something! To end conversation, say 'BYE'. (Press the ENTER key again to stop recording)".bright_red());
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

        println!();

    }

    Ok(())
}

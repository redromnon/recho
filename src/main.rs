mod tts;
mod stt;
use tts::{TTSModel};
use stt::{AudioRecorder, STTModel};
use colored::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut recorder = AudioRecorder::new();
    let mut speech_recogniser = STTModel::new("whisper/ggml-small.bin");
    let mut piper_tts = TTSModel::new("piper/en_US-arctic-medium.onnx.json", 12);

    loop  {

        println!("Press ENTER to begin recording...");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        println!("{}", "Recording... Press ENTER to stop.\nOr SAY BYE to end conversation.".yellow());
        recorder.start_recording().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let (samples, source_sample_rate) = recorder.stop_recording();
        println!("Captured {} samples", samples.len());

        let transcribed_text = speech_recogniser.transcribe(samples, source_sample_rate);

        println!("{}", transcribed_text);

        let trimmed_input = transcribed_text.trim();

        if trimmed_input.to_lowercase().contains("bye") {
            let bye_text = "Goodbye! Have a nice day!";
            println!("{}", bye_text.blue());
            piper_tts.process_text(bye_text);
            break;
        }else {
            piper_tts.process_text(trimmed_input);
        }
    }

    Ok(())
}

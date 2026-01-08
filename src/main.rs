mod tts;
mod stt;
use tts::{TTSModel};
use stt::{AudioRecorder, STTModel};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut recorder = AudioRecorder::new();
    let mut speech_recogniser = STTModel::new("whisper/ggml-small-q8_0.bin");
    let mut piper_tts = TTSModel::new("piper/en_GB-cori-medium.onnx.json");

    loop  {

        println!("Press Enter to start recording...");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        println!("[||] Recording... Press Enter to stop.");
        recorder.start_recording().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let (samples, source_sample_rate) = recorder.stop_recording();
        println!("Captured {} samples", samples.len());

        let transcribed_text = speech_recogniser.transcribe(samples, source_sample_rate);

        println!("{}", transcribed_text);

        let trimmed_input = transcribed_text.trim();

        if trimmed_input.to_lowercase().contains("stop") {
            println!("Exiting. Goodbye!");
            break;
        }else {
            piper_tts.process_text(trimmed_input);
        }
    }

    Ok(())
}

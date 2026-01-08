use std::io;
mod tts;
mod stt;
use tts::{load_tts_model, process_text};
use stt::{AudioRecorder, STTModel};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut recorder = AudioRecorder::new();
    let mut speech_recogniser = STTModel::new("whisper/ggml-small-q8_0.bin");

    println!("Press Enter to start recording...");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    println!("Recording... Press Enter to stop.");
    recorder.start_recording().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let (samples, source_sample_rate) = recorder.stop_recording();
    println!("Captured {} samples", samples.len());

    let transcribed_text = speech_recogniser.transcribe(samples, source_sample_rate);

    println!("{:?}", transcribed_text);

    // let tts_config_path = "piper/en_GB-cori-medium.onnx.json";

    // let mut synth = load_tts_model(tts_config_path);

    // //Get text from user
    // loop  {

    //     println!("Enter text to speak:");

    //     // Read input from the user
    //     let mut input = String::new();
    //     io::stdin().read_line(&mut input).expect("Failed to read line");

    //     // Trim whitespace and check if it's "stop"
    //     let trimmed_input = input.trim();

    //     match trimmed_input {
    //         "stop" => {
    //             println!("Exiting. Goodbye!");
    //             break;
    //         }
    //         _ => {
    //                 process_text(trimmed_input, &mut synth);
    //         }
    //     }
    // }

    Ok(())
}

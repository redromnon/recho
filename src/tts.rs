use piper_rs::synth::{PiperSpeechSynthesizer};
use std::time::{Instant};
use rodio::{OutputStreamBuilder, Sink, buffer::SamplesBuffer};
use std::path::Path;

pub fn load_tts_model(config_path: &str) -> PiperSpeechSynthesizer{

    // Load the model
    let start: Instant;
    start = Instant::now();
    let model = piper_rs::from_config_path(Path::new(config_path)).unwrap();

    // Create a single synthesizer that owns the model
    let synth = PiperSpeechSynthesizer::new(model).unwrap();

    println!("Model loaded in: {:?}", start.elapsed());

    synth

}

fn play_samples(samples: Vec<f32>, sample_rate: u32) {
    // Open default output stream
    let stream = OutputStreamBuilder::open_default_stream()
        .expect("Failed to open default stream");

    // Create a sink attached to the stream mixer
    let sink = Sink::connect_new(&stream.mixer());

    // Create audio source from your samples (mono)
    let source = SamplesBuffer::new(1, sample_rate, samples);

    // Append and play
    sink.append(source);

    // Wait until finished
    sink.sleep_until_end();
}

pub fn process_text(text: &str, synth: &mut PiperSpeechSynthesizer) {

    let start: Instant;
    start = Instant::now();

    // Prepare a buffer for samples
    let mut samples: Vec<f32> = Vec::new();

    let audio = synth.synthesize_parallel(text.to_string(), None).unwrap();
    println!("Text processed in: {:?}", start.elapsed());

    for result in audio {
        samples.append(&mut result.unwrap().into_vec());
    }
    play_samples(samples.to_vec(), 22050)
}
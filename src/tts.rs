use piper_rs::synth::{PiperSpeechSynthesizer};
use std::time::{Instant};
use rodio::{OutputStream, OutputStreamBuilder, Sink, buffer::SamplesBuffer};
use std::path::Path;
use colored::*;

pub struct TTSModel {
    synth: PiperSpeechSynthesizer,
    sink: Sink,
    _stream: OutputStream
}

impl TTSModel {
    
    pub fn new(config_path: &str, speaker_id: u8) -> Self {

        //Audio stream setup
        let _stream = OutputStreamBuilder::open_default_stream()
            .expect("Failed to open default stream");

        // Create a sink attached to the stream mixer
        let sink = Sink::connect_new(&_stream.mixer());

        // Load the model
        let start: Instant;
        start = Instant::now();
        let model = piper_rs::from_config_path(Path::new(config_path)).unwrap();        

        //Check if many speakers exist
        let speakers = model
        .get_speakers()
        .unwrap()
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();

        if speakers.len() > 1 {
            let sid = speaker_id as i64;
            model.set_speaker(sid);
            println!("Selected speaker id: {}", speaker_id);
        }

        // Create a single synthesizer that owns the model
        let synth = PiperSpeechSynthesizer::new(model).unwrap();

        println!("{} {:?}", "TTS model loaded in:".blue(), start.elapsed());

        Self { synth, sink, _stream }
    }

    fn play_samples(&mut self, samples: Vec<f32>) {
        // // Open default output stream
        // let stream = OutputStreamBuilder::open_default_stream()
        //     .expect("Failed to open default stream");

        // // Create a sink attached to the stream mixer
        // let sink = Sink::connect_new(&stream.mixer());

        // Create audio source from your samples (mono)
        let source = SamplesBuffer::new(1, 22050, samples);

        // Append and play
        self.sink.append(source);

        // Wait until finished
        self.sink.sleep_until_end();
    }

    pub fn process_text(&mut self, text: &str) {

        let start: Instant;
        start = Instant::now();

        // Prepare a buffer for samples
        let mut samples: Vec<f32> = Vec::new();

        let audio = self.synth.synthesize_parallel(
            text.to_string(), 
            None
        
        ).unwrap();
        println!("{} {:?}", "Text processed in:".green(), start.elapsed());

        for result in audio {
            samples.append(&mut result.unwrap().into_vec());
        }

        self.play_samples(samples);


    }

}
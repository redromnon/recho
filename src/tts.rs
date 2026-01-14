use std::time::{Instant};
use rodio::{OutputStream, OutputStreamBuilder, Sink, buffer::SamplesBuffer};
use std::path::Path;
use colored::*;
use crate::tts_helper::{load_text_to_speech, load_voice_style, TextToSpeech, Style};

pub struct TTSModel {
    model: TextToSpeech,
    voice_style: Style,
    sink: Sink,
    _stream: OutputStream, //Required to keep output loaded due to ownership and access to sound card
}

impl TTSModel {
    
    pub fn new(onnx_path: std::path::PathBuf, voice_style_path: std::path::PathBuf) -> Self {

        //Audio stream setup
        let _stream = OutputStreamBuilder::open_default_stream()
            .expect("Failed to open default stream");

        //Create a sink attached to the stream mixer
        let sink = Sink::connect_new(&_stream.mixer());

        //Load the model
        let start: Instant;
        start = Instant::now();
        const USE_GPU: bool = false;
        let onnx_dir = Path::new(&onnx_path).to_str().unwrap();
        let model = load_text_to_speech(onnx_dir, USE_GPU).unwrap();        

        //Add voice
        let voice_style_paths = voice_style_path.to_str().unwrap();
        let voice_style = load_voice_style(&[voice_style_paths.to_string()], true).unwrap();

        println!("{} {:?}", "TTS model loaded in:".blue(), start.elapsed());

        Self { model, voice_style, sink, _stream }
    }

    fn play_samples(&mut self, samples: Vec<f32>, sample_rate: u32) {

        //Create audio source from your samples (mono)
        let source = SamplesBuffer::new(1, sample_rate, samples);

        //Append and play
        self.sink.append(source);

        //Wait until finished
        self.sink.sleep_until_end();

    }

    pub fn process_text(&mut self, text: &str) {

        let start= Instant::now();

        //Destructure the tuple returned by the model
        let (wav, duration): (Vec<f32>, f32) = self.model.call(
            text, 
            "en", 
            &self.voice_style, 
            10, 
            1.2, 
            0.3
        ).unwrap();

        //Create the trimmed version
        let actual_len = (self.model.sample_rate as f32 * duration) as usize;
        
        //Use .to_vec() to turn the slice back into an owned Vec<f32>
        let wav_trimmed = wav[..actual_len.min(wav.len())].to_vec();

        println!("{} {:?}", "Speech processed in:".green(), start.elapsed());

        self.play_samples(wav_trimmed, self.model.sample_rate as u32);


    }

}
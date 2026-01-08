use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters, WhisperState};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use samplerate::{convert, ConverterType};
use std::time::{Instant};

pub struct AudioRecorder {
    // We store the stream because the recording stops when the stream is dropped
    stream: Option<cpal::Stream>,
    data: Arc<Mutex<Vec<i16>>>,
    source_sample_rate: u32,
}

impl AudioRecorder {

    pub fn new() -> Self {
        Self {
            stream: None,
            data: Arc::new(Mutex::new(Vec::new())),
            source_sample_rate: 44100, // Default fallback
        }
    }

    /// Function 1: Start Recording
    pub fn start_recording(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let host = cpal::default_host();
        let device = host.default_input_device()
            .ok_or("No input device found")?;
        
        let config = device.default_input_config()?;
        let data = Arc::clone(&self.data);
        let channels = config.channels() as usize;
        self.source_sample_rate = device.default_input_config()?.sample_rate().0;

        // Clear previous recordings
        data.lock().unwrap().clear();

        let stream = device.build_input_stream(
            &config.into(),
            move |input_data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut buffer = data.lock().unwrap();
                // We only take the first channel to ensure Mono
                for frame in input_data.chunks(channels as usize) {
                    let sample = (frame[0].clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
                    buffer.push(sample);
                }
            },
            |err| eprintln!("Stream error: {}", err),
            None
        )?;

        stream.play()?;
        self.stream = Some(stream);
        Ok(())
    }

    /// Function 2: Stop Recording and return the samples
    pub fn stop_recording(&mut self) -> (Vec<i16>, u32) {
        // Dropping the stream stops the recording
        self.stream = None;
        
        let mut buffer = self.data.lock().unwrap();
        let raw_samples = buffer.clone();
        buffer.clear();
        
        (raw_samples, self.source_sample_rate)
    }

}


pub struct STTModel {
    state: WhisperState,
}

impl STTModel {

    pub fn new(model_path: &str) -> Self {

        //Load model
        let start: Instant;
        start = Instant::now();
        let ctx = WhisperContext::new_with_params(model_path, WhisperContextParameters::default())
            .expect("Failed to load model");
        let state = ctx.create_state().expect("Failed to create state");
        println!("Whisper model loaded in: {:?}", start.elapsed());
        Self { state }
    }

    pub fn transcribe(&mut self, samples: Vec<i16>, source_sample_rate: u32) -> String {

        // the sampling strategy will determine how accurate your final output is going to be
        // typically BeamSearch is more accurate at the cost of significantly increased CPU time
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

        // and set the language to translate to as english
        params.set_language(Some("en"));

        // we also explicitly disable anything that prints to stdout
        // despite all of this you will still get things printing to stdout,
        // be prepared to deal with it
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        // we must convert to 16KHz mono f32 samples for the model
        // some utilities exist for this
        // note that you don't need to use these, you can do it yourself or any other way you want
        // these are just provided for convenience
        let mut inter_samples = vec![Default::default(); samples.len()];
        //let _mono_samples = vec![Default::default(); samples.len() / 2];

        whisper_rs::convert_integer_to_float_audio(&samples, &mut inter_samples)
            .expect("failed to convert audio data");
        
        //Resample to 16,000Hz (Whisper's requirement)
        if source_sample_rate != 16000 {
            inter_samples = convert(
                source_sample_rate,
                16000,
                1, // Mono
                ConverterType::SincBestQuality,
                &inter_samples,
            ).expect("Resampling failed");
        }
        

        //Run model
        self.state
            .full(params, &inter_samples[..])
            .expect("failed to run model");

        //Fetch segments
        let mut full_text = String::new();
        for segment in self.state.as_iter() {
            // println!(
            //     "[{} - {}]: {}",
            //     // these timestamps are in centiseconds (10s of milliseconds)
            //     segment.start_timestamp(),
            //     segment.end_timestamp(),
            //     // this default Display implementation will result in any invalid UTF-8
            //     // being converted into the Unicode replacement character, U+FFFD
            //     segment
            // );
            full_text.push_str(segment.to_str().unwrap());
        }

        full_text

    }

}
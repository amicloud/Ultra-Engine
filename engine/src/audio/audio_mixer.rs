use cpal::{
    Host,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use rtrb::Producer;

pub struct AudioMixer {
    host: Host,
    device: cpal::Device,
    stream: Option<cpal::Stream>,
    producer: Option<Producer<f32>>,
}

impl Default for AudioMixer {
    fn default() -> Self {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output device available");
        let mut s = Self {
            host,
            device,
            stream: None,
            producer: None,
        };
        s.build_stream();
        s
    }
}

impl AudioMixer {
    fn build_stream(&mut self) {
        let config = self.device.default_output_config().unwrap();
        let channels = config.channels() as usize;
        let (producer, mut consumer) = rtrb::RingBuffer::<f32>::new(48_000);

        let stream = self
            .device
            .build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    for frame in data.chunks_mut(channels) {
                        let sample = consumer.pop().unwrap_or(0.0);
                        for sample_out in frame.iter_mut() {
                            *sample_out = sample;
                        }
                    }
                },
                move |err| eprintln!("Stream error: {}", err),
                None, // None=blocking, Some(Duration)=timeout
            )
            .expect("failed to build output stream");

        self.producer = Some(producer);
        self.stream = Some(stream);
    }

    fn move_to_buffer(&mut self, samples: &[f32]) {
        let producer = self.producer.as_mut().expect("No producer.");
        for &sample in samples {
            let _ = producer.push(sample);
        }
    }

    fn play(&self) {
        self.stream
            .as_ref()
            .expect("No stream.")
            .play()
            .expect("failed to play stream");
    }

    fn pause(&self) {
        self.stream
            .as_ref()
            .expect("No stream.")
            .pause()
            .expect("failed to pause stream");
    }
}

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub fn start_listening(threshold: f32, status: std::sync::Arc<std::sync::atomic::AtomicBool>) {
  let host = cpal::default_host();
  let device = host.default_input_device().expect("No input device found");
  let config = device.default_input_config().unwrap();

  let stream = device.build_input_stream(
    &config.into(),
    move |data: &[f32], _: &_| {
      let rms = (data.iter().map(|&s| s * s).sum::<f32>() / data.len() as f32).sqrt();
      status.store(rms > threshold, std::sync::atomic::Ordering::Relaxed);
    },
    |err| eprintln!("Audio error: {}", err),
    None
  ).unwrap();

  stream.play().unwrap();
  std::thread::park();
}
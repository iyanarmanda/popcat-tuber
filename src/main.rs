mod audio;
mod ui;
// mod debug;

use std::sync::{Arc, atomic::AtomicBool};
use std::thread;

fn main() -> anyhow::Result<()> {
  let is_talking = Arc::new(AtomicBool::new(false));
    
  let audio_status = Arc::clone(&is_talking);
  thread::spawn(move || {
    audio::start_listening(0.05, audio_status);
  });

  println!("Starting audio log...");
//  debug::run_stdout_mode(Arc::clone(&is_talking));
  ui::run_ui(Arc::clone(&is_talking));

  Ok(())
}

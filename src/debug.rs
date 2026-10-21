use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::io::{self, Write};

pub fn run_stdout_mode(is_talking: Arc<AtomicBool>) {
  let mut last_state = false;

  println!("--- PopCat State Monitor ---");
  println!("Ctrl+C to stop");

  loop {
    let current_state = is_talking.load(Ordering::Relaxed);

    if current_state != last_state {
      if current_state {
        println!("POP!");
      } else {
        println!("...");
      }

      io::stdout().flush().unwrap();
      last_state = current_state;
    }

    std::thread::sleep(std::time::Duration::from_millis(50));
  }
}

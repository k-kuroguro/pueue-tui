use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::{Mutex, OnceLock};

struct Logger {
   file: Mutex<File>,
}

impl Logger {
   fn new() -> Self {
      let file = OpenOptions::new()
         .create(true)
         .append(true)
         .open("a.txt")
         .expect("failed to open a.txt");

      Logger {
         file: Mutex::new(file),
      }
   }

   fn log(&self, msg: &str) {
      let mut file = self.file.lock().unwrap();
      writeln!(file, "{}", msg).unwrap();
   }
}

static LOGGER: OnceLock<Logger> = OnceLock::new();

fn logger() -> &'static Logger {
   LOGGER.get_or_init(|| Logger::new())
}

pub fn log(msg: &str) {
   logger().log(msg);
}

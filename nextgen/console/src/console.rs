use std::io::{self, BufWriter, Write};
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, RwLock};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;

#[derive(Clone, Copy)]
pub enum ConsoleTarget {
    Stderr,
    Stdout,
}

pub struct Console {
    buffer: Arc<RwLock<BufWriter<Vec<u8>>>>,
    channel: Sender<bool>,
    handle: JoinHandle<()>,
    target: ConsoleTarget,
    quiet: bool,
}

impl Console {
    pub fn new(target: ConsoleTarget, quiet: bool) -> Self {
        let buffer = Arc::new(RwLock::new(BufWriter::new(Vec::new())));
        let buffer_clone = Arc::clone(&buffer);
        let (tx, rx) = mpsc::channel();

        // Every 100ms, flush the buffer
        let handle = spawn(move || loop {
            if quiet {
                break;
            }

            sleep(Duration::from_millis(100));

            if let Ok(mut out) = buffer_clone.write() {
                flush(&mut out, target).unwrap();

                // Has the thread been closed?
                match rx.try_recv() {
                    // If false, no
                    Ok(value) if value == false => {}
                    // If true or an error, yes
                    _ => {
                        break;
                    }
                }
            } else {
                break;
            }
        });

        Self {
            buffer,
            channel: tx,
            handle,
            target,
            quiet,
        }
    }

    pub fn close(self) {
        if let Ok(mut out) = self.buffer.write() {
            flush(&mut out, self.target).unwrap();
        }

        self.channel.send(true).unwrap();
        self.handle.join().unwrap();
    }

    pub fn write(&self, data: Vec<u8>) {
        if self.quiet {
            return;
        }

        let mut buffer = self
            .buffer
            .write()
            .expect("Failed to acquire console write lock.");

        buffer.write_all(&data).unwrap();

        // Buffer has written its data to the vec, so flush it
        if buffer.get_ref().len() > 0 {
            flush(&mut buffer, self.target).unwrap();
        }
    }

    pub fn write_line(&self, mut data: Vec<u8>) {
        data.push(b'\n');
        self.write(data);
    }
}

fn flush(buffer: &mut BufWriter<Vec<u8>>, target: ConsoleTarget) -> io::Result<()> {
    buffer.flush()?;

    let data = buffer.get_mut().drain(0..).collect::<Vec<_>>();

    if data.is_empty() {
        return Ok(());
    }

    match target {
        ConsoleTarget::Stderr => io::stderr().lock().write_all(&data),
        ConsoleTarget::Stdout => io::stdout().lock().write_all(&data),
    }
}


extern crate argh;
extern crate logged_fu_skater as lfs;

use argh::FromArgs;
use std::io::{self, Read};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::{thread, time};
use std::thread::JoinHandle;

#[derive(FromArgs)]
/// Obfuscates the input string into a human readable hash.
struct Args {
    /// how many bytes of padding. 0-8, default 0
    #[argh(option, short = 'p', default = "0")]
    padding: u8,
    /// the input to obfuscate
    #[argh(positional, default = "String::new()")]
    input: String,
}

fn main() {
    let args: Args = argh::from_env();
    let result = match &args.input[..] {
        "" => {
            let (receiver, handle) = spawn_stdin_channel();
            sleep(100);
            match receiver.try_recv() {
                Ok(key) => {
                    if let Err(_) = handle.join() {
                        panic!("Couldn't kill thread.")
                    }
                    key
                },
                Err(TryRecvError::Empty) => {
                    panic!("No input given and nothing to read in stdin (pipe).");
                },
                Err(TryRecvError::Disconnected) => {
                    panic!("Channel disconnected");
                },
            }
        },
        _ => args.input
    };
    println!("{}", lfs::obfp(result.trim(), args.padding));
}

fn spawn_stdin_channel() -> (Receiver<String>, JoinHandle<()>) {
    let (tx, rx) = mpsc::channel::<String>();
    let handle = thread::spawn(move || {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    (rx, handle)
}

fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}

/// This program is to learn basic about Command
// use std::env;
use std::process;
use std::process::Command;
use std::str;

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");

    let command = Command::new("pidof")
        .arg("command_0001")
        .output()
        .expect("Failed to execute command");

    if command.status.success() {
        println!("command {:?}", command);
        match str::from_utf8(&command.stdout) {
            Ok(pid) => {
                let pid: u32 = pid.to_string().trim().parse().unwrap();
                println!("pids: {} vs {}", pid, process::id());
                assert_eq!(process::id(), pid.to_string().trim().parse().unwrap());
            }
            Err(e) => {
                println!("Err: {:?}", e);
            }
        }
    }
}

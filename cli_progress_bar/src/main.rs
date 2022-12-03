use std::{thread, time};
fn main() {
    let pb = indicatif::ProgressBar::new(100);
    let ten_millis = time::Duration::from_millis(10);
    for _ in 0..100 {
        pb.inc(1);
        thread::sleep(ten_millis);
    }
    pb.finish_with_message("Done.");
}
